use crate::errors::{GraderError, GraderResult};
use crate::instance;
use crate::instance::{Instance, RunVerdict};
use crate::submission::result::*;
use crate::utils::{get_base_path, get_code_extension, get_env, get_message};
use futures::sink::{Sink, SinkExt};
use manifest::Manifest;
use std::{io::Write, path::Path, path::PathBuf, process::Command};
use tokio::fs;

pub mod manifest;
pub mod result;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum SubmissionStatus {
    Initialized,
    TaskNotFound,
    Compiling,
    Compiled,
    CompilationError(String),
    Running(u64),
    Done(SubmissionResult),
}
impl Default for SubmissionStatus {
    fn default() -> Self {
        SubmissionStatus::Initialized
    }
}

#[derive(Debug)]
pub enum SubmissionMessage {
    Status(SubmissionStatus),
    RunResult(RunResult),
    GroupResult(GroupResult),
}
impl Default for SubmissionMessage {
    fn default() -> Self {
        SubmissionMessage::Status(SubmissionStatus::Initialized)
    }
}

#[derive(Default)]
pub struct Submission<T> {
    pub task_id: String,
    pub submission_id: String,
    pub language: String,
    pub code_path: Vec<PathBuf>,
    pub task_manifest: Manifest,
    pub tmp_path: PathBuf,
    pub task_path: PathBuf,
    pub bin_path: PathBuf,
    pub message_handler: T,
}

impl<T> std::fmt::Display for Submission<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Submission {} {} {} {:?} {:?} {:?} {:?} {:?}",
            self.task_id,
            self.submission_id,
            self.language,
            self.code_path,
            self.task_manifest,
            self.tmp_path,
            self.task_path,
            self.bin_path
        )
    }
}

impl<T> std::fmt::Debug for Submission<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Submission {} {} {} {:?} {:?} {:?} {:?} {:?}",
            self.task_id,
            self.submission_id,
            self.language,
            self.code_path,
            self.task_manifest,
            self.tmp_path,
            self.task_path,
            self.bin_path
        )
    }
}

impl<T> Submission<T> {
    pub async fn try_from(
        task_id: impl ToString,
        submission_id: impl ToString,
        language: impl ToString,
        code: &[String],
        mut message_handler: T,
    ) -> GraderResult<Self>
    where
        T: Sink<SubmissionMessage> + std::marker::Unpin,
    {
        let task_id = task_id.to_string();
        let submission_id = submission_id.to_string();
        let language = language.to_string();
        let tmp_path = PathBuf::from(get_env("TEMPORARY_PATH")).join(&submission_id);
        fs::remove_dir_all(&tmp_path).await.ok();
        fs::create_dir(&tmp_path).await?;
        let extension = get_code_extension(&language);
        let task_path = get_base_path().join("tasks").join(&task_id);

        if task_path.is_dir() == false {
            _ = message_handler
                .send(SubmissionMessage::Status(SubmissionStatus::TaskNotFound))
                .await;
            return Err(GraderError::task_not_found());
        }

        if task_path.join("compile_files").is_dir() {
            let mut entries = fs::read_dir(task_path.join("compile_files")).await?;
            while let Some(entry) = entries.next_entry().await? {
                let path = entry;
                fs::copy(&path.path(), tmp_path.join(&path.file_name())).await?;
            }
        }
        Ok(Submission {
            task_id,
            submission_id,
            language,
            code_path: code
                .iter()
                .enumerate()
                .map(|(idx, val)| {
                    let code_path =
                        tmp_path.join(format!("code_{}.{}", &idx.to_string(), &extension));
                    let mut file = std::fs::File::create(&code_path)?;
                    file.write_all(val.as_bytes())?;

                    Ok(code_path)
                })
                .collect::<GraderResult<Vec<_>>>()?,
            task_manifest: Manifest::from(task_path.join("manifest.yaml"))?,
            tmp_path,
            task_path,
            bin_path: PathBuf::new(),
            message_handler,
        })
    }

    pub async fn compile(&mut self) -> GraderResult<bool>
    where
        T: Sink<SubmissionMessage> + std::marker::Unpin,
    {
        _ = self
            .message_handler
            .send(SubmissionMessage::Status(SubmissionStatus::Compiling))
            .await;

        let compiler_path = get_base_path()
            .join("scripts")
            .join("compile_scripts")
            .join(&self.language);

        let mut args = vec![&self.tmp_path];
        self.code_path.iter().for_each(|path| {
            args.push(&path);
        });

        let mut tmp_compile_files = vec![];

        if let Some(compile_files) = &self.task_manifest.compile_files {
            for compile_file in compile_files
                .get(&self.language)
                .ok_or(GraderError::invalid_index())?
            {
                tmp_compile_files.push(self.tmp_path.join(&compile_file));
            }
        }

        tmp_compile_files.iter().for_each(|path| {
            args.push(&path);
        });

        log::debug!("compiler path: {compiler_path:?} args: {args:?}");

        let compile_output = dbg!(Command::new(compiler_path).args(args).output()?);
        let compile_output_args = String::from_utf8(compile_output.stdout.clone())?
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let return_code: i32 = compile_output_args
            .get(0)
            .map_or(1, |s| s.parse::<i32>().unwrap_or(1));

        match return_code {
            0 => {
                _ = self
                    .message_handler
                    .send(SubmissionMessage::Status(SubmissionStatus::Compiled))
                    .await;
            }
            _ => {
                _ = self
                    .message_handler
                    .send(SubmissionMessage::Status(
                        SubmissionStatus::CompilationError(String::from_utf8(
                            compile_output.stdout,
                        )?),
                    ))
                    .await;
            }
        }

        if return_code == 0 {
            self.bin_path = PathBuf::from(
                compile_output_args
                    .get(1)
                    .ok_or(GraderError::invalid_index())?,
            );
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn run_each(
        &mut self,
        checker: &Path,
        runner: &Path,
        index: u64,
    ) -> GraderResult<RunResult>
    where
        T: Sink<SubmissionMessage> + std::marker::Unpin,
    {
        _ = self
            .message_handler
            .send(SubmissionMessage::Status(SubmissionStatus::Running(index)))
            .await;
        let input_path = self
            .task_path
            .join("testcases")
            .join(format!("{}.in", index));
        let output_path = self.tmp_path.join(format!("output_{}", index));
        let sol_path = self
            .task_path
            .join("testcases")
            .join(format!("{}.sol", index));

        let mut instance = instance! {
            time_limit: self.task_manifest.time_limit.ok_or(GraderError::invalid_value())?,
            memory_limit: self.task_manifest.memory_limit.ok_or(GraderError::invalid_value())? * 1000,
            bin_path: self.bin_path.clone(),
            input_path: input_path.clone(),
            output_path: output_path.clone(),
            runner_path: runner.to_path_buf(),
            box_id: self.submission_id.clone().parse::<u64>().unwrap()
        };

        instance.init().await?;
        
        let instance_result = instance.run().await?;

        let mut run_result = RunResult::from(
            self.submission_id.to_owned(),
            index,
            instance_result.time_usage,
            instance_result.memory_usage,
        );

        run_result.status = match instance_result.status {
            RunVerdict::VerdictOK => {
                let args = vec![&input_path, &output_path, &sol_path];
                log::debug!("{input_path:?}, {output_path:?}, {sol_path:?}");
                let checker_result = Command::new(&checker).args(args).output()?;
                log::debug!("{checker_result:?}\n");
                let checker_output = String::from_utf8(checker_result.stdout)?
                    .trim_end_matches('\n')
                    .lines()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();

                run_result.score = checker_output
                    .get(1)
                    .ok_or(GraderError::invalid_index())?
                    .parse()?;
                run_result.message = checker_output
                    .get(2)
                    .map_or(String::new(), |v| v.to_owned());

                checker_output
                    .get(0)
                    .ok_or(GraderError::invalid_index())?
                    .as_str()
                    .to_owned()
            }
            RunVerdict::VerdictTLE => String::from("Time Limit Exceeded"),
            RunVerdict::VerdictMLE => String::from("Memory Limit Exceeded"),
            RunVerdict::VerdictRE => String::from("Runtime Error"),
            RunVerdict::VerdictSG => String::from("Signal Error"),
            _ => String::from("Judge Error"),
        };

        if run_result.message.is_empty() {
            run_result.message = get_message(&run_result.status);
        }

        _ = self
            .message_handler
            .send(SubmissionMessage::RunResult(run_result.clone()))
            .await;
        Ok(run_result)
    }

    pub async fn run(&mut self) -> GraderResult<SubmissionResult>
    where
        T: Sink<SubmissionMessage> + std::marker::Unpin,
    {
        if self.bin_path == PathBuf::new() {
            return Ok(SubmissionResult {
                score: 0.0,
                full_score: 0.0,
                submission_id: self.submission_id.clone(),
                group_result: vec![],
            });
        }

        let checker =
            self.task_manifest
                .checker
                .as_ref()
                .map_or(self.task_path.join("checker"), |file| {
                    get_base_path()
                        .join("scripts")
                        .join("checker_scripts")
                        .join(&file)
                });
        let grouper =
            self.task_manifest
                .grouper
                .as_ref()
                .map_or(self.task_path.join("grouper"), |file| {
                    get_base_path()
                        .join("scripts")
                        .join("grouper_scripts")
                        .join(&file)
                });
        let runner = get_base_path()
            .join("scripts")
            .join("runner_scripts")
            .join(&self.language);

        let mut last_test = 1;
        let mut total_score: f64 = 0.0;
        let mut total_full_score: f64 = 0.0;
        let mut group_results = Vec::new();
        for (group_index, (full_score, tests)) in
            self.task_manifest.groups.clone().iter().enumerate()
        {
            total_full_score += full_score;

            let mut skip = false;
            let mut args = vec![full_score.to_string()];

            let mut group_result = GroupResult::from(
                *full_score,
                self.submission_id.to_owned(),
                (group_index + 1) as u64,
            );
            for index in last_test..(last_test + tests) {
                let run_result = if skip {
                    RunResult::from(self.submission_id.to_owned(), index, 0.0, 0)
                } else {
                    self.run_each(&checker, &runner, index).await?
                };
                args.push(run_result.score.to_string());
                skip = &run_result.status != "Correct" && &run_result.status != "Partially Correct";

                group_result.run_result.push(run_result);
            }
            if !skip {
                let grouper_result = Command::new(&grouper).args(args).output()?;
                group_result.score = String::from_utf8(grouper_result.stdout)?
                    .trim_end_matches('\n')
                    .parse()?;

                total_score += group_result.score;
            }
            _ = self
                .message_handler
                .send(SubmissionMessage::GroupResult(group_result.clone()))
                .await;

            group_results.push(group_result);

            last_test += tests;
        }

        let submission_result = SubmissionResult {
            score: total_score,
            full_score: total_full_score,
            submission_id: self.submission_id.to_owned(),
            group_result: group_results,
        };
        _ = self
            .message_handler
            .send(SubmissionMessage::Status(SubmissionStatus::Done(
                submission_result.clone(),
            )))
            .await;
        Ok(submission_result)
    }
}

impl<T> Drop for Submission<T> {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.tmp_path).ok();
    }
}
