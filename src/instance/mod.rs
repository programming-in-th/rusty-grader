use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(test)]
mod tests;

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

pub fn get_env(name: &'static str) -> String {
    env::var(name).expect(&format!("cannot get {} from env", name))
}

impl Instance {
    pub fn get_arguments(&self) -> Vec<String> {
        let mut args: Vec<String> = vec![
            String::from("-b"),
            self.box_id.to_string(),
            String::from("-M"),
            self.log_file.to_str().unwrap().to_string(),
            String::from("-t"),
            self.time_limit.to_string(),
            String::from("-w"),
            (self.time_limit + 5.0).to_string(),
            String::from("-x"),
            (self.time_limit + 1.0).to_string(),
            String::from("-i"),
            String::from("input"),
            String::from("-o"),
            String::from("output"),
            String::from("--run"),
            String::from("--"),
            String::from("runner"),
            String::from("--cg"),
            String::from("--cg-timing"),
            String::from("--processes=128"),
            format!("--cg-mem={}", self.memory_limit),
        ];

        if Path::new(&get_env("ALTERNATIVE_PATH")).is_dir() {
            args.push(format!("--dir={}", get_env("ALTERNATIVE_PATH")));
        }
        args
    }

    pub fn check_root_permission(&self) {
        let permission_result = Command::new("id").arg("-u").output();
        match permission_result {
            Ok(output) => {
                let output_string = String::from_utf8(output.stdout).unwrap();
                assert_eq!(output_string.trim(), "0", "isolate must be run as root");
            }
            _ => panic!("unable to get current user id",),
        }
    }

    pub fn get_result(&self) -> InstanceResult {
        let log_content = fs::read_to_string(&self.log_file).expect("Unable to open log file");
        let mut result: InstanceResult = Default::default();
        for log_line in log_content.split("\n") {
            let args: Vec<&str> = log_line.split(":").collect();
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
                    "max-rss" => result.memory_usage = args[1].parse().unwrap(),
                    _ => (),
                }
            }
        }
        if result.memory_usage > self.memory_limit && result.status == Default::default() {
            result.status = RunVerdict::VerdictMLE;
        }
        result
    }

    pub fn init(&mut self) {
        self.check_root_permission();

        for tmp_box_idx in 1..=1000 {
            let box_path = Command::new(get_env("ISOLATE_PATH"))
                .args(&["--init", "--cg", "-b"])
                .arg(tmp_box_idx.to_string())
                .output()
                .expect("Unable to run isolate --init command.");
            if box_path.status.success() {
                let mut box_path = String::from_utf8(box_path.stdout).unwrap();
                box_path = box_path.strip_suffix("\n").unwrap_or(&box_path).to_string();
                self.box_path = PathBuf::from(&box_path).join("box");
                self.box_id = tmp_box_idx;
                break;
            }
        }

        let tmp_path = get_env("TEMPORARY_PATH");
        self.log_file = PathBuf::from(tmp_path).join(format!("tmp_log_{}.txt", self.box_id));

        fs::copy(&self.input_path, &self.box_path.join("input"))
            .expect("Unable to copy input file into box directory");

        fs::copy(
            &self.bin_path,
            &self.box_path.join(self.bin_path.file_name().unwrap()),
        )
        .expect("Unable to copy user exec file into box directory");

        fs::copy(&self.runner_path, &self.box_path.join("runner"))
            .expect("Unable to copy runner script into box directory");
    }

    pub fn run(&self) -> InstanceResult {
        let args = self.get_arguments();
        Command::new(get_env("ISOLATE_PATH"))
            .args(args)
            .output()
            .expect("Unable to run isolate.");

        self.get_result()
    }

    pub fn cleanup(&self) {
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
