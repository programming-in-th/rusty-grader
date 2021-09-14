macro_rules! s {
    ($x:expr) => {
        String::from($x);
    };
}
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::instance::error::InstanceError;

#[derive(Default, Debug)]
pub struct Instance {
    pub box_path: PathBuf,
    pub log_file: PathBuf,
    pub box_id: u32,
    pub bin_path: PathBuf,
    pub time_limit: f64,
    pub memory_limit: u64,
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub runner_path: PathBuf,
}

pub enum RunVerdict {
    OK,
    TLE,
    MLE,
    RE,
    XX,
    OT,
}

pub struct InstanceResult {
    pub status: RunVerdict,
    pub time_usage: u32,
    pub memory_usage: u32,
}

fn get_env(name: &'static str) -> Result<String, InstanceError> {
    env::var(name)
        .map_err(|_| InstanceError::EnvorimentError(format!("cannot get {} from env", name)))
}

impl Instance {
    fn get_arguments(&self) -> Result<Vec<String>, InstanceError> {
        let mut args: Vec<String> = Vec::new();
        args.push(s!("--cg"));
        args.push(s!("--cg-timing"));
        args.push(s!("--processes=128"));

        args.push(s!("-b"));
        args.push(self.box_id.to_string());

        args.push(s!("-M"));
        args.push(self.log_file.to_str().unwrap().to_string());

        args.push(s!("-t"));
        args.push(self.time_limit.to_string());

        args.push(format!("--cg-mem={}", self.memory_limit));

        args.push(s!("-w"));
        args.push((self.time_limit + 5.0).to_string());

        args.push(s!("-x"));
        args.push((self.time_limit + 1.0).to_string());

        let alternative_path = get_env("ALTERNATIVE_PATH")?;

        if Path::new(&alternative_path).is_dir() {
            args.push(format!("--dir={}", alternative_path));
        }

        args.push(s!("-i"));
        args.push(s!("input"));

        args.push(s!("-o"));
        args.push(s!("output"));

        args.push(s!("--run"));
        args.push(s!("--"));
        args.push(s!("runner"));

        Ok(args)
    }

    fn check_root_permission(&self) -> Result<(), InstanceError> {
        let permission_result = Command::new("id").arg("-u").output();
        match permission_result {
            Ok(output) => {
                let output_string = String::from_utf8(output.stdout).unwrap();
                if output_string.trim() == "0" {
                    Ok(())
                } else {
                    Err(InstanceError::PermissionError(s!(
                        "isolate must be run as root"
                    )))
                }
            }
            _ => Err(InstanceError::PermissionError(s!(
                "unable to get current user id"
            ))),
        }
    }

    pub fn init(&mut self) -> Result<(), InstanceError> {
        self.check_root_permission()?;

        let box_path = Command::new(get_env("ISOLATE_PATH")?)
            .args(&["--init", "--cg", "-b"])
            .arg(self.box_id.to_string())
            .output()
            .map_err(|_| {
                InstanceError::PermissionError(s!("Unable to run isolate --init command."))
            })?;

        self.box_path = PathBuf::from(
            String::from_utf8(box_path.stdout)
                .unwrap()
                .strip_suffix("\n")
                .unwrap(),
        )
        .join("box");

        let tmp_path = get_env("TEMPORARY_PATH")?;
        self.log_file = PathBuf::from(tmp_path).join(format!("tmp_log_{}.txt", self.box_id));

        Command::new("cp")
            .arg(self.input_path.to_str().unwrap())
            .arg(self.box_path.join("input").to_str().unwrap())
            .output()
            .map_err(|_| {
                InstanceError::PermissionError(s!("Unable to copy input file into box directory"))
            })?;

        Command::new("cp")
            .arg(self.bin_path.to_str().unwrap())
            .arg(self.box_path.to_str().unwrap())
            .output()
            .map_err(|_| {
                InstanceError::PermissionError(s!(
                    "Unable to copy user exec file into box directory"
                ))
            })?;
        Command::new("cp")
            .arg(self.runner_path.to_str().unwrap())
            .arg(self.box_path.join("runner").to_str().unwrap())
            .output()
            .map_err(|_| {
                InstanceError::PermissionError(s!(
                    "Unable to copy runner script into box directory"
                ))
            })?;
        Ok(())
    }

    pub fn run(&self) -> Result<InstanceResult, InstanceError> {
        let args = self.get_arguments()?;
        let box_output = Command::new(get_env("ISOLATE_PATH")?)
            .args(args)
            .output()
            .map_err(|_| InstanceError::PermissionError(s!("Unable to run isolate.")))?;

        Ok(InstanceResult {
            status: RunVerdict::OK,
            time_usage: 1,
            memory_usage: 1,
        })
    }

    pub fn cleanup(&self) -> Result<(), InstanceError> {
        Command::new(get_env("ISOLATE_PATH")?)
            .args(&["--cleanup", "--cg", "-b"])
            .arg(self.box_id.to_string())
            .output()
            .map_err(|_| {
                InstanceError::PermissionError(s!("Unable to cleanup isolate --cleanup command."))
            })?;

        Command::new("rm")
            .arg(self.log_file.to_str().unwrap())
            .output()
            .map_err(|_| InstanceError::PermissionError(s!("Unable to remove log file.")))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::instance::error::InstanceError;
    use crate::instance::model::Instance;
    use dotenv::dotenv;
    use std::env;
    use std::path::PathBuf;
    use std::process::Command;

    #[test]
    fn declare_variable() -> Result<(), InstanceError> {
        let instance = Instance {
            box_id: 1,
            bin_path: PathBuf::from("/path/to/bin"),
            time_limit: 1.0,
            memory_limit: 512000,
            input_path: PathBuf::from("/path/to/in"),
            output_path: PathBuf::from("/path/to/out"),
            runner_path: PathBuf::from("/path/to/runner"),
            ..Default::default()
        };
        Ok(())
    }

    #[test]
    fn initialize_instance() -> Result<(), InstanceError> {
        dotenv().ok();
        // get base directory
        let base_dir = PathBuf::from(env::current_dir().unwrap())
            .join("tests")
            .join("instance");

        // compile cpp first
        Command::new(base_dir.join("compile_cpp").to_str().unwrap())
            .arg(base_dir.to_str().unwrap())
            .arg(base_dir.join("a_plus_b.cpp").to_str().unwrap())
            .output()
            .map_err(|_| InstanceError::PermissionError(s!("Unable to compile file")))?;

        let mut instance = Instance {
            box_id: 1,
            bin_path: base_dir.join("bin"),
            time_limit: 1.0,
            memory_limit: 512000,
            input_path: base_dir.join("input.txt"),
            output_path: base_dir.join("output.txt"),
            runner_path: base_dir.join("run_cpp"),
            ..Default::default()
        };

        instance.init()?;

        instance.run()?;

        instance.cleanup()?;

        // clean up
        Command::new("rm")
            .arg(base_dir.join("bin").to_str().unwrap())
            .arg(base_dir.join("compileMsg").to_str().unwrap())
            .output()
            .map_err(|_| InstanceError::PermissionError(s!("Unable to compile file")))?;

        Ok(())
    }
}
