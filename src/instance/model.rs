use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::instance::error::InstanceError;

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

pub fn get_env(name: &'static str) -> Result<String, InstanceError> {
    env::var(name)
        .map_err(|_| InstanceError::EnvironmentError(format!("cannot get {} from env", name)))
}

impl Instance {
    pub fn get_arguments(&self) -> Result<Vec<String>, InstanceError> {
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

        if Path::new(&get_env("ALTERNATIVE_PATH")?).is_dir() {
            args.push(format!("--dir={}", get_env("ALTERNATIVE_PATH")?));
        }

        Ok(args)
    }

    pub fn check_root_permission(&self) -> Result<(), InstanceError> {
        let permission_result = Command::new("id").arg("-u").output();
        match permission_result {
            Ok(output) => {
                let output_string = String::from_utf8(output.stdout).unwrap();
                if output_string.trim() == "0" {
                    Ok(())
                } else {
                    Err(InstanceError::PermissionError(
                        "isolate must be run as root",
                    ))
                }
            }
            _ => Err(InstanceError::PermissionError(
                "unable to get current user id",
            )),
        }
    }

    pub fn get_result(&self) -> Result<InstanceResult, InstanceError> {
        let log_content = fs::read_to_string(&self.log_file)
            .map_err(|_| InstanceError::PermissionError("Unable to open log file"))?;
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
        if result.memory_usage > self.memory_limit {
            result.status = RunVerdict::VerdictMLE;
        }
        Ok(result)
    }

    pub fn init(&mut self) -> Result<(), InstanceError> {
        self.check_root_permission()?;

        for tmp_box_idx in 1..=1000 {
            let box_path = Command::new(get_env("ISOLATE_PATH")?)
                .args(&["--init", "--cg", "-b"])
                .arg(tmp_box_idx.to_string())
                .output()
                .map_err(|_| {
                    InstanceError::PermissionError("Unable to run isolate --init command.")
                })?;
            
            if box_path.status.success() {
                let mut box_path = String::from_utf8(box_path.stdout).unwrap();
                box_path = box_path.strip_suffix("\n").unwrap_or(&box_path).to_string();
                self.box_path = PathBuf::from(&box_path).join("box");
                self.box_id = tmp_box_idx;
                break;
            }
        }

        let tmp_path = get_env("TEMPORARY_PATH")?;
        self.log_file = PathBuf::from(tmp_path).join(format!("tmp_log_{}.txt", self.box_id));

        fs::copy(&self.input_path, &self.box_path.join("input")).map_err(|_| {
            InstanceError::PermissionError("Unable to copy input file into box directory")
        })?;

        fs::copy(
            &self.bin_path,
            &self.box_path.join(self.bin_path.file_name().unwrap()),
        )
        .map_err(|_| {
            InstanceError::PermissionError("Unable to copy user exec file into box directory")
        })?;

        fs::copy(&self.runner_path, &self.box_path.join("runner")).map_err(|_| {
            InstanceError::PermissionError("Unable to copy runner script into box directory")
        })?;

        Ok(())
    }

    pub fn run(&self) -> Result<InstanceResult, InstanceError> {
        let args = self.get_arguments()?;
        Command::new(get_env("ISOLATE_PATH")?)
            .args(args)
            .output()
            .map_err(|_| InstanceError::PermissionError("Unable to run isolate."))?;

        self.get_result()
    }

    pub fn cleanup(&self) -> Result<(), InstanceError> {
        Command::new(get_env("ISOLATE_PATH")?)
            .args(&["--cleanup", "--cg", "-b"])
            .arg(self.box_id.to_string())
            .output()
            .map_err(|_| {
                InstanceError::PermissionError("Unable to cleanup isolate --cleanup command.")
            })?;

        if self.log_file.is_file() {
            fs::remove_file(&self.log_file)
                .map_err(|_| InstanceError::PermissionError("Unable to remove log file."))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[test]
    fn initialize_instance() -> Result<(), InstanceError> {
        dotenv().ok();
        // get base directory
        let base_dir = PathBuf::from(env::current_dir().unwrap())
            .join("tests")
            .join("instance");

        let tmp_dir = PathBuf::from(get_env("TEMPORARY_PATH")?).join("init_instance");

        if !tmp_dir.is_dir() {
            fs::create_dir(&tmp_dir)
                .map_err(|_| InstanceError::PermissionError("Unable to create tmp directory"))?;
        }

        Command::new(&base_dir.join("compile_cpp"))
            .arg(&tmp_dir)
            .arg(&base_dir.join("a_plus_b.cpp"))
            .output()
            .map_err(|_| InstanceError::PermissionError("Unable to compile file"))?;

        let mut instance = Instance {
            bin_path: tmp_dir.join("bin"),
            time_limit: 1.0,
            memory_limit: 512000,
            input_path: base_dir.join("input.txt"),
            output_path: base_dir.join("output.txt"),
            runner_path: base_dir.join("run_cpp"),
            ..Default::default()
        };

        let result = || -> Result<(), InstanceError> {
            instance.init()?;
            instance.run()?;
            Ok(())
        }();

        // clean up
        fs::remove_dir_all(&tmp_dir)
            .map_err(|_| InstanceError::PermissionError("Unable to remove tmp directory"))?;

        instance.cleanup()?;
        result
    }

    #[test]
    fn should_error_if_input_path_is_wrong() -> Result<(), InstanceError> {
        dotenv().ok();
        // get base directory
        let base_dir = PathBuf::from(env::current_dir().unwrap())
            .join("tests")
            .join("instance");

        let tmp_dir = PathBuf::from(get_env("TEMPORARY_PATH")?).join("test_input_path_is_wrong");

        if !tmp_dir.is_dir() {
            fs::create_dir(&tmp_dir)
                .map_err(|_| InstanceError::PermissionError("Unable to create tmp directory"))?;
        }

        // compile cpp first
        Command::new(&base_dir.join("compile_cpp"))
            .arg(&tmp_dir)
            .arg(&base_dir.join("a_plus_b.cpp"))
            .output()
            .map_err(|_| InstanceError::PermissionError("Unable to compile files"))?;

        let mut instance = Instance {
            bin_path: tmp_dir.join("bin"),
            time_limit: 1.0,
            memory_limit: 512000,
            input_path: base_dir.join("input.txt_wrong_path"),
            output_path: base_dir.join("output.txt"),
            runner_path: base_dir.join("run_cpp"),
            ..Default::default()
        };

        let init_result = instance.init();

        fs::remove_dir_all(&tmp_dir)
            .map_err(|_| InstanceError::PermissionError("Unable to remove tmp directory"))?;

        instance.cleanup()?;

        assert_eq!(
            init_result,
            Err(InstanceError::PermissionError(
                "Unable to copy input file into box directory"
            ))
        );
        Ok(())
    }

    #[test]
    fn should_error_if_output_path_is_wrong() -> Result<(), InstanceError> {
        dotenv().ok();
        // get base directory
        let base_dir = PathBuf::from(env::current_dir().unwrap())
            .join("tests")
            .join("instance");

        let tmp_dir = PathBuf::from(get_env("TEMPORARY_PATH")?).join("test_output_path_is_wrong");

        if !tmp_dir.is_dir() {
            fs::create_dir(&tmp_dir)
                .map_err(|_| InstanceError::PermissionError("Unable to create tmp directory"))?;
        }

        // compile cpp first
        Command::new(&base_dir.join("compile_cpp"))
            .arg(&tmp_dir)
            .arg(&base_dir.join("a_plus_b.cpp"))
            .output()
            .map_err(|_| InstanceError::PermissionError("Unable to compile files"))?;

        let mut instance = Instance {
            bin_path: tmp_dir.join("bin_wrong_path"),
            time_limit: 1.0,
            memory_limit: 512000,
            input_path: base_dir.join("input.txt"),
            output_path: base_dir.join("output.txt"),
            runner_path: base_dir.join("run_cpp"),
            ..Default::default()
        };


        let init_result = instance.init();

        fs::remove_dir_all(&tmp_dir)
            .map_err(|_| InstanceError::PermissionError("Unable to remove tmp directory"))?;

        instance.cleanup()?;

        assert_eq!(
            init_result,
            Err(InstanceError::PermissionError(
                "Unable to copy user exec file into box directory"
            ))
        );
        Ok(())
    }

    #[test]
    fn should_error_if_runner_path_is_wrong() -> Result<(), InstanceError> {
        dotenv().ok();
        // get base directory
        let base_dir = PathBuf::from(env::current_dir().unwrap())
            .join("tests")
            .join("instance");

        let tmp_dir = PathBuf::from(get_env("TEMPORARY_PATH")?).join("test_runner_path_is_wrong");

        if !tmp_dir.is_dir() {
            fs::create_dir(&tmp_dir)
                .map_err(|_| InstanceError::PermissionError("Unable to create tmp directory"))?;
        }

        // compile cpp first
        Command::new(&base_dir.join("compile_cpp"))
            .arg(&tmp_dir)
            .arg(&base_dir.join("a_plus_b.cpp"))
            .output()
            .map_err(|_| InstanceError::PermissionError("Unable to compile files"))?;

        let mut instance = Instance {
            bin_path: tmp_dir.join("bin"),
            time_limit: 1.0,
            memory_limit: 512000,
            input_path: base_dir.join("input.txt"),
            output_path: base_dir.join("output.txt"),
            runner_path: base_dir.join("run_cpp_wrong_path"),
            ..Default::default()
        };

        let init_result = instance.init();

        fs::remove_dir_all(&tmp_dir)
            .map_err(|_| InstanceError::PermissionError("Unable to remove tmp directory"))?;

        instance.cleanup()?;

        assert_eq!(
            init_result,
            Err(InstanceError::PermissionError(
                "Unable to copy runner script into box directory"
            ))
        );
        Ok(())
    }

    #[test]
    fn should_read_log_correctly_when_ok() -> Result<(), InstanceError> {
        dotenv().ok();
        // get base directory
        let base_dir = PathBuf::from(env::current_dir().unwrap())
            .join("tests")
            .join("instance");

        let instance = Instance {
            log_file: base_dir.join("log_ok.txt"),
            memory_limit: 4000,
            ..Default::default()
        };

        let result = instance.get_result()?;

        assert_eq!(
            result,
            InstanceResult {
                status: RunVerdict::VerdictOK,
                time_usage: 0.004,
                memory_usage: 3196,
            }
        );
        Ok(())
    }

    #[test]
    fn should_trigger_when_read_log_with_memory_limit_exceeded() -> Result<(), InstanceError> {
        dotenv().ok();
        // get base directory
        let base_dir = PathBuf::from(env::current_dir().unwrap())
            .join("tests")
            .join("instance");

        let instance = Instance {
            log_file: base_dir.join("log_ok.txt"),
            memory_limit: 1000,
            ..Default::default()
        };

        let result = instance.get_result()?;

        assert_eq!(
            result,
            InstanceResult {
                status: RunVerdict::VerdictMLE,
                time_usage: 0.004,
                memory_usage: 3196,
            }
        );
        Ok(())
    }
}
