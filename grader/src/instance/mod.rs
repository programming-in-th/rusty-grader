use crate::combine_argument;
use crate::utils::get_env;
use std::{fs, path::PathBuf, process::Command};

#[cfg(test)]
mod tests;

/// Instance define a single test case to run in isolated environment
#[derive(Default, Debug)]
pub struct Instance {
    pub box_path: PathBuf,
    pub log_file: PathBuf,
    pub box_id: u64,
    pub bin_path: PathBuf,
    pub time_limit: f64,
    pub memory_limit: u64,
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub runner_path: PathBuf,
}

impl Drop for Instance {
    fn drop(&mut self) {
        Command::new(get_env("ISOLATE_PATH"))
            .args(&["--cleanup", "--cg", "-b"])
            .arg(self.box_id.to_string())
            .output()
            .expect("Unable to cleanup isolate --cleanup command.");

        if self.log_file.is_file() {
            fs::remove_file(&self.log_file).expect("Unable to remove log file.");
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RunVerdict {
    VerdictOK,
    VerdictTLE,
    VerdictMLE,
    VerdictRE,
    VerdictXX,
    VerdictSG,
}

impl Default for RunVerdict {
    fn default() -> Self {
        Self::VerdictOK
    }
}

#[derive(Default, PartialEq, Debug)]
pub struct InstanceResult {
    pub status: RunVerdict,
    pub time_usage: f64,
    pub memory_usage: u64,
}

impl Instance {
    fn get_run_arguments(&self) -> Vec<String> {
        combine_argument![
            "-b",
            self.box_id.to_string(),
            "-M",
            self.log_file.to_str().unwrap().to_string(),
            "-t",
            self.time_limit.to_string(),
            "-w",
            (self.time_limit + 5.0).to_string(),
            "-x",
            (self.time_limit + 1.0).to_string(),
            "-i",
            "input",
            "-o",
            "output",
            "--processes=128",
            "--cg",
            "--cg-timing",
            format!("--cg-mem={}", self.memory_limit),
            format!("--dir={}", get_env("ALTERNATIVE_PATH")),
            "--run",
            "--",
            "runner"
        ]
    }

    pub fn get_result(&self) -> InstanceResult {
        let log_content = fs::read_to_string(&self.log_file).unwrap();
        let mut result: InstanceResult = Default::default();
        let mut memory_limit_exceeded = false;
        for log_line in log_content.lines() {
            let args: Vec<&str> = log_line.split(':').collect();
            if args.len() >= 2 {
                match &*args[0] {
                    "status" => {
                        result.status = match &*args[1] {
                            "RE" => RunVerdict::VerdictRE,
                            "SG" => RunVerdict::VerdictSG,
                            "TO" => RunVerdict::VerdictTLE,
                            "XX" => RunVerdict::VerdictXX,
                            _ => RunVerdict::VerdictSG,
                        }
                    }
                    "time" => result.time_usage = args[1].parse().unwrap(),
                    "cg-mem" => result.memory_usage = args[1].parse().unwrap(),
                    "cg-oom-killed" => memory_limit_exceeded = args[1].trim() == "1",
                    _ => (),
                }
            }
        }
        if memory_limit_exceeded || result.memory_usage >= self.memory_limit && result.status == Default::default() {
            result.status = RunVerdict::VerdictMLE;
        }
        result
    }

    pub fn init(&mut self) {
        for tmp_box_idx in 1..=1000 {
            let box_path = Command::new(get_env("ISOLATE_PATH"))
                .args(&["--init", "--cg", "-b"])
                .arg(tmp_box_idx.to_string())
                .output()
                .unwrap();

            if box_path.status.success() {
                let box_path = String::from_utf8(box_path.stdout).unwrap();
                self.box_path = PathBuf::from(box_path.trim_end_matches('\n')).join("box");
                self.box_id = tmp_box_idx;
                break;
            }
        }

        let tmp_path = get_env("TEMPORARY_PATH");
        self.log_file = PathBuf::from(tmp_path).join(format!("tmp_log_{}.txt", self.box_id));

        fs::copy(&self.input_path, &self.box_path.join("input")).unwrap();

        fs::copy(
            &self.bin_path,
            &self.box_path.join(self.bin_path.file_name().unwrap()),
        )
        .unwrap();

        fs::copy(&self.runner_path, &self.box_path.join("runner")).unwrap();
    }

    pub fn run(&self) -> InstanceResult {
        let args = self.get_run_arguments();
        Command::new(get_env("ISOLATE_PATH"))
            .args(args)
            .output()
            .unwrap();

        let result = self.get_result();
        if result.status == RunVerdict::VerdictOK {
            fs::copy(&self.box_path.join("output"), &self.output_path).unwrap();
        }
        result
    }
}
