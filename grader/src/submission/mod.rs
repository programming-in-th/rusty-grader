use crate::instance;
use crate::instance::{Instance, InstanceResult, RunVerdict};
use crate::utils::{get_base_path, get_code_extension, get_env};
use manifest::Manifest;
use std::{
    fs, io,
    io::{Error, Write},
    path::PathBuf,
    process::Command,
};

pub mod manifest;

#[cfg(test)]
mod tests;

#[derive(Default, Debug, Clone)]
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

enum TestCaseVerdict {
    VerdictCorrect,
    VerdictIncorrect,
    VerdictPCorrect,
    VerdictTLE,
    VerdictMLE,
    VerdictRE,
    VerdictXX,
    VerdictSG,
}

pub struct RunResult {
    status: TestCaseVerdict,
    score: f64,
    message: String,
}

impl Submission {
    pub fn init(&mut self) -> io::Result<()> {
        self.tmp_path = PathBuf::from(get_env("TEMPORARY_PATH")).join(&self.submission_id);
        fs::create_dir(&self.tmp_path)?;

        let extension = get_code_extension(&self.language);
        for (idx, code) in self.code.iter().enumerate() {
            let code_path = self
                .tmp_path
                .join(format!("code_{}.{}", &idx.to_string(), &extension));
            let mut file = fs::File::create(&code_path)?;
            file.write(code.as_bytes())?;

            self.code_path.push(code_path.clone());
        }

        self.task_path = get_base_path().join("tasks").join(&self.task_id);
        self.task_manifest = Manifest::from(self.task_path.join("manifest.yaml"));

        if self.task_path.join("compile_files").is_dir() {
            let entries = fs::read_dir(self.task_path.join("compile_files"))?;
            for entry in entries {
                let path = entry?;
                fs::copy(&path.path(), self.tmp_path.join(&path.file_name()))?;
            }
        }
        Ok(())
    }

    pub fn compile(&mut self) -> io::Result<()> {
        let compiler_path = get_base_path()
            .join("scripts")
            .join("compile_scripts")
            .join(&self.language);

        let mut args = vec![self.tmp_path.clone()];
        args = args
            .iter()
            .cloned()
            .chain(self.code_path.iter().cloned())
            .collect();

        if let Some(compile_files) = &self.task_manifest.compile_files {
            for compile_file in compile_files.get(&self.language).unwrap() {
                args.push(self.task_path.join(&compile_file));
            }
        }

        let compile_output = Command::new(compiler_path).args(args).output()?;
        let compile_output_args = String::from_utf8(compile_output.stdout)
            .unwrap()
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let return_code: i32 = compile_output_args.get(0).unwrap().parse().unwrap();

        if return_code != 0 {
            return Err(Error::from_raw_os_error(return_code));
        }

        self.bin_path = PathBuf::from(compile_output_args.get(1).unwrap());
        Ok(())
    }

    fn run_each(&self, checker: &PathBuf, runner: &PathBuf, index: u64) -> io::Result<RunResult> {
        let mut instance = instance! {
            time_limit: self.task_manifest.time_limit.unwrap(),
            memory_limit: self.task_manifest.memory_limit.unwrap(),
            bin_path: self.bin_path.clone(),
            input_path: self.task_path.join("testcases").join(format!("{}.in", index)),
            output_path: self.tmp_path.join(format!("output_{}", index)),
            runner_path: runner.clone()
        };

        instance.init()?;
        let result = instance.run()?;

        Ok(())
    }

    pub fn run(&self) -> io::Result<()> {
        if (!self.task_manifest.output_only) {
            let checker =
                self.task_manifest
                    .checker
                    .clone()
                    .map_or(self.task_path.join("checker"), |file| {
                        get_base_path()
                            .join("scripts")
                            .join("checker_script")
                            .join(&file)
                    });
            let grouper =
                self.task_manifest
                    .grouper
                    .clone()
                    .map_or(self.task_path.join("grouper"), |file| {
                        get_base_path()
                            .join("scripts")
                            .join("grouper_script")
                            .join(&file)
                    });
            let runner = get_base_path()
                .join("scripts")
                .join("checker_script")
                .join(&self.language);

            let mut last_test = 1;

            for (full_score, tests) in &self.task_manifest.groups {
                for index in last_test..(last_test + tests) {
                    self.run_each(&checker, &runner, index);
                }

                last_test += tests;
            }
        }

        Ok(())
    }
}

impl Drop for Submission {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.tmp_path).expect("Unable to remove submission folder.");
    }
}
