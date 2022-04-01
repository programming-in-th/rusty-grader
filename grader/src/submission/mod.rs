use crate::instance;
use crate::instance::{Instance, RunVerdict};
use crate::s;
use crate::submission::result::*;
use crate::utils::{get_base_path, get_code_extension, get_env, get_message};
use manifest::Manifest;
use std::{fs, io::Write, path::Path, path::PathBuf, process::Command};
use crate::errors::{GraderError, GraderResult};

pub mod manifest;
pub mod result;

// #[cfg(test)]
// mod tests;

#[derive(Debug)]
pub enum SubmissionStatus {
    Initialized,
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
pub trait DisplayFnT: FnMut(SubmissionMessage) {}

impl<F> DisplayFnT for F where F: FnMut(SubmissionMessage) {}

impl std::fmt::Debug for dyn DisplayFnT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DisplayFunction")
    }
}
pub type DisplayFn<'a> = Box<dyn DisplayFnT + 'a>;
#[derive(Default)]
pub struct Submission<'a> {
    pub task_id: String,
    pub submission_id: String,
    pub language: String,
    pub code_path: Vec<PathBuf>,
    pub task_manifest: Manifest,
    pub tmp_path: PathBuf,
    pub task_path: PathBuf,
    pub bin_path: PathBuf,
    pub message_handler: Option<DisplayFn<'a>>,
}

impl<'a> std::fmt::Display for Submission<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl<'a> Submission<'a> {
    pub fn from(
        task_id: String,
        submission_id: String,
        language: String,
        code: &[String],
        message_handler: Option<DisplayFn<'a>>,
    ) -> GraderResult<Self> {
        let tmp_path = PathBuf::from(get_env("TEMPORARY_PATH")).join(&submission_id);
        fs::create_dir(&tmp_path)?;
        let extension = get_code_extension(&language);
        let task_path = get_base_path().join("tasks").join(&task_id);
        if task_path.join("compile_files").is_dir() {
            let entries = fs::read_dir(task_path.join("compile_files"))?;
            for entry in entries {
                let path = entry?;
                fs::copy(&path.path(), tmp_path.join(&path.file_name()))?;
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
                    let mut file = fs::File::create(&code_path)?;
                    file.write_all(val.as_bytes())?;

                    Ok(code_path)
                })
                .collect::<GraderResult<Vec<_>>>()?,
            task_manifest: Manifest::from(task_path.join("manifest.yaml")),
            tmp_path,
            task_path,
            bin_path: PathBuf::new(),
            message_handler,
        })
    }

    pub fn compile(&mut self) -> GraderResult<()> {
        if let Some(message_handler) = &mut self.message_handler {
            message_handler(SubmissionMessage::Status(SubmissionStatus::Compiling))
        }
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

        let compile_output = Command::new(compiler_path).args(args).output()?;
        let compile_output_args = String::from_utf8(compile_output.stdout.clone())?
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let return_code: i32 = compile_output_args
            .get(0)
            .ok_or(GraderError::invalid_index())?
            .parse()?;

        // if return_code != 0 {
        //     return Err(Error::from_raw_os_error(return_code));
        // }

        self.bin_path = PathBuf::from(
            compile_output_args
                .get(1)
                .ok_or(GraderError::invalid_index())?,
        );

        if let Some(message_handler) = &mut self.message_handler {
            match return_code {
                0 => message_handler(SubmissionMessage::Status(SubmissionStatus::Compiled)),
                _ => message_handler(SubmissionMessage::Status(
                    SubmissionStatus::CompilationError(String::from_utf8(compile_output.stdout)?),
                )),
            }
        }
        Ok(())
    }

    fn run_each(
        &mut self,
        checker: &Path,
        runner: &Path,
        index: u64,
    ) -> GraderResult<RunResult> {
        if let Some(message_handler) = &mut self.message_handler {
            message_handler(SubmissionMessage::Status(SubmissionStatus::Running(index)))
        }
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
            runner_path: runner.to_path_buf()
        };

        instance.init()?;
        let instance_result = instance.run()?;

        let mut run_result = RunResult::from(
            self.submission_id.to_owned(),
            index,
            instance_result.time_usage,
            instance_result.memory_usage,
        );

        run_result.status = match instance_result.status {
            RunVerdict::VerdictOK => {
                let args = vec![&input_path, &output_path, &sol_path];
                let checker_result = Command::new(&checker).args(args).output()?;
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
            RunVerdict::VerdictTLE => s!("Time Limit Exceeded"),
            RunVerdict::VerdictMLE => s!("Memory Limit Exceeded"),
            RunVerdict::VerdictRE => s!("Runtime Error"),
            RunVerdict::VerdictSG => s!("Signal Error"),
            _ => s!("Judge Error"),
        };

        if run_result.message.is_empty() {
            run_result.message = get_message(&run_result.status);
        }

        if let Some(message_handler) = &mut self.message_handler {
            message_handler(SubmissionMessage::RunResult(run_result.clone()))
        }
        Ok(run_result)
    }

    pub fn run(&mut self) -> GraderResult<SubmissionResult> {
        // if !self.task_manifest.output_only {
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
        let mut total_full_score: u64 = 0;
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
                    self.run_each(&checker, &runner, index)?
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
            if let Some(message_handler) = &mut self.message_handler {
                message_handler(SubmissionMessage::GroupResult(group_result.clone()));
            }
            group_results.push(group_result);

            last_test += tests;
        }

        let submission_result = SubmissionResult {
            score: total_score,
            full_score: total_full_score,
            submission_id: self.submission_id.to_owned(),
            group_result: group_results,
        };
        if let Some(message_handler) = &mut self.message_handler {
            message_handler(SubmissionMessage::Status(SubmissionStatus::Done(
                submission_result.clone(),
            )));
        }
        Ok(submission_result)
    }
}

impl<'a> Drop for Submission<'a> {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.tmp_path).expect("Unable to remove submission folder.");
    }
}
