use crate::instance;
use crate::instance::{Instance, RunVerdict};
use crate::s;
use crate::submission::result::*;
use crate::utils::{get_base_path, get_code_extension, get_env, get_message};
use manifest::Manifest;
use std::{fs, io::Write, path::Path, path::PathBuf, process::Command};

pub mod manifest;
pub mod result;

#[cfg(test)]
mod tests;

#[derive(Default, Debug)]
pub struct Submission {
    pub task_id: String,
    pub submission_id: String,
    pub language: String,
    pub code: Vec<String>,
    pub code_path: Vec<PathBuf>,
    pub task_manifest: Manifest,
    pub tmp_path: PathBuf,
    pub task_path: PathBuf,
    pub bin_path: PathBuf,
}

impl Submission {
    pub fn init(&mut self) {
        self.tmp_path = PathBuf::from(get_env("TEMPORARY_PATH")).join(&self.submission_id);
        fs::create_dir(&self.tmp_path).unwrap();

        let extension = get_code_extension(&self.language);
        for (idx, code) in self.code.iter().enumerate() {
            let code_path = self
                .tmp_path
                .join(format!("code_{}.{}", &idx.to_string(), &extension));
            let mut file = fs::File::create(&code_path).unwrap();
            file.write_all(code.as_bytes()).unwrap();

            self.code_path.push(code_path);
        }

        self.task_path = get_base_path().join("tasks").join(&self.task_id);
        self.task_manifest = Manifest::from(self.task_path.join("manifest.yaml"));

        if self.task_path.join("compile_files").is_dir() {
            let entries = fs::read_dir(self.task_path.join("compile_files")).unwrap();
            for entry in entries {
                let path = entry.unwrap();
                fs::copy(&path.path(), self.tmp_path.join(&path.file_name())).unwrap();
            }
        }
    }

    pub fn compile(&mut self) {
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
            for compile_file in compile_files.get(&self.language).unwrap() {
                tmp_compile_files.push(self.task_path.join(&compile_file));
            }
        }

        tmp_compile_files.iter().for_each(|path| {
            args.push(&path);
        });

        let compile_output = Command::new(compiler_path).args(args).output().unwrap();
        let compile_output_args = String::from_utf8(compile_output.stdout)
            .unwrap()
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        // let return_code: i32 = compile_output_args.get(0).unwrap().parse().unwrap();

        // if return_code != 0 {
        //     return Err(Error::from_raw_os_error(return_code));
        // }

        self.bin_path = PathBuf::from(compile_output_args.get(1).unwrap());
    }

    fn run_each(&self, checker: &Path, runner: &Path, index: u64) -> RunResult {
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
            time_limit: self.task_manifest.time_limit.unwrap(),
            memory_limit: self.task_manifest.memory_limit.unwrap() * 1000,
            bin_path: self.bin_path.clone(),
            input_path: input_path.clone(),
            output_path: output_path.clone(),
            runner_path: runner.to_path_buf()
        };

        instance.init();
        let instance_result = instance.run();

        let mut run_result = RunResult::from(
            &self.submission_id,
            index,
            instance_result.time_usage,
            instance_result.memory_usage,
        );

        run_result.status = match instance_result.status {
            RunVerdict::VerdictOK => {
                let args = vec![&input_path, &output_path, &sol_path];
                let checker_result = Command::new(&checker).args(args).output().unwrap();
                let checker_output = String::from_utf8(checker_result.stdout)
                    .unwrap()
                    .trim_end_matches('\n')
                    .lines()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();

                run_result.score = checker_output.get(1).unwrap().parse().unwrap();
                run_result.message = checker_output
                    .get(2)
                    .map_or(String::new(), |v| v.to_owned());
                checker_output.get(0).unwrap().as_str().to_owned()
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

        run_result
    }

    pub fn run(&self) -> SubmissionResult {
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

        for (group_index, (full_score, tests)) in self.task_manifest.groups.iter().enumerate() {
            total_full_score += full_score;

            let mut skip = false;
            let mut args = vec![full_score.to_string()];

            let mut group_result =
                GroupResult::from(*full_score, &self.submission_id, (group_index + 1) as u64);
            for index in last_test..(last_test + tests) {
                let run_result = if skip {
                    RunResult::from(&self.submission_id, index, 0.0, 0)
                } else {
                    self.run_each(&checker, &runner, index)
                };
                args.push(run_result.score.to_string());
                skip = &run_result.status != "Correct" && &run_result.status != "Partially Correct";

                group_result.run_result.push(run_result);
            }
            if !skip {
                let grouper_result = Command::new(&grouper).args(args).output().unwrap();
                group_result.score = String::from_utf8(grouper_result.stdout)
                    .unwrap()
                    .trim_end_matches('\n')
                    .parse()
                    .unwrap();

                total_score += group_result.score;
            }
            group_results.push(group_result);

            last_test += tests;
        }

        SubmissionResult {
            score: total_score,
            full_score: total_full_score,
            submission_id: self.submission_id.to_owned(),
            group_result: group_results,
        }
    }
}

impl Drop for Submission {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.tmp_path).expect("Unable to remove submission folder.");
    }
}
