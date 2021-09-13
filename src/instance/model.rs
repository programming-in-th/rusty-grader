use std::env;
use std::path::{ Path, PathBuf };
use std::process::Command;

use crate::instance::error::InstanceError;

pub struct Instance {
    box_path: PathBuf,
    box_id: u32,
    bin_path: PathBuf,
    log_file: PathBuf,
    time_limit: f64,
    memory_limit: u64,
    input_path: PathBuf,
    output_path: PathBuf,
    runner_path: PathBuf,
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
    status: RunVerdict,
    time_usage: u32,
    memory_usage: u32,
}

fn get_env(name: &'static str) -> Result<String, InstanceError> {
    env::var(name)
        .map_err(|_| InstanceError::EnvorimentError(format!("cannot get {} from env", name)))
}

impl Instance {
    pub fn new(
        box_id: u32,
        bin_path: PathBuf,
        log_file: PathBuf,
        time_limit: f64,
        memory_limit: u64,
        input_path: PathBuf,
        output_path: PathBuf,
        runner_path: PathBuf,
    ) -> Self {
        Self {
            box_id,
            bin_path,
            log_file,
            time_limit,
            memory_limit,
            input_path,
            output_path,
            runner_path,
            box_path: PathBuf::from(""),
        }
    }

    fn get_arguments(&self) -> Result<Vec<String>, InstanceError> {
        let mut args: Vec<String> = Vec::new();
        args.push("--cg".to_string());
        args.push("--cg-timing".to_string());
        args.push("--processes=128".to_string());

        args.push("-b".to_string());
        args.push(self.box_id.to_string());

        args.push("-M".to_string());
        args.push(self.log_file.to_str().unwrap().to_string());

        args.push("-t".to_string());
        args.push(self.time_limit.to_string());

        args.push(format!("--cg-mem={}", self.memory_limit));

        args.push("-w".to_string());
        args.push((self.time_limit + 5.0).to_string());

        args.push("-x".to_string());
        args.push((self.time_limit + 1.0).to_string());

        let alternative_path = get_env("ALTERNATIVE_PATH")?;

        if Path::new(&alternative_path).is_dir() {
            args.push(format!("--dir={}", alternative_path));
        }

        args.push("-i".to_string());
        args.push(self.input_path.to_str().unwrap().to_string());

        args.push("-o".to_string());
        args.push(self.output_path.to_str().unwrap().to_string());

        Ok(args)
    }

    fn check_root_permission() -> Result<(), InstanceError> {
        let permission_result = Command::new("id").arg("-u").output();
        match permission_result {
            Ok(output) => {
                let output_string = String::from_utf8(output.stdout).unwrap();
                if output_string.trim() == "0" {
                    Ok(())
                } else {
                    Err(InstanceError::PermissionError(String::from("isolate must be run as root")))
                }
            },
            _ => Err(InstanceError::PermissionError(String::from("unable to get current user id")))
        }
    }

    pub fn init(&mut self) -> Result<(), InstanceError> {
        // self.check_root_permission()?;

        let isolate_path = get_env("ISOLATE_PATH")?;

        let box_path = Command::new(isolate_path)
            .arg("--cg")
            .arg("-b")
            .arg(self.box_id.to_string())
            .arg("--init")
            .output().map_err(|_|InstanceError::PermissionError(
                String::from("Unable to run isolate --init command. Does a box already exist? If so, you must clean up first.")
            ))?;

        self.box_path = PathBuf::from(String::from_utf8(box_path.stdout).unwrap()).join("box");

        Command::new("cp")
            .arg(self.input_path.to_str().unwrap())
            .arg(self.box_path.join("input").to_str().unwrap())
            .output()
            .map_err(|_| InstanceError::PermissionError(
                String::from("Unable to copy input file into box directory")
            ))?;
        Command::new("cp")
            .arg(self.bin_path.to_str().unwrap())
            .arg(self.box_path.to_str().unwrap())
            .output()
            .map_err(|_| InstanceError::PermissionError(
                String::from("Unable to copy user exec file into box directory")
            ))?;
        Command::new("cp")
            .arg(self.runner_path.to_str().unwrap())
            .arg(self.box_path.to_str().unwrap())
            .output()
            .map_err(|_| InstanceError::PermissionError(
                String::from("Unable to copy runner script into box directory")
            ))?;
        Ok(())
    }

    pub fn run(&self) -> Result<InstanceResult, InstanceError> {
        let arg = self.get_arguments();
        Err(InstanceError::PermissionError(String::new()))
    }

    pub fn cleanup() -> Result<(), InstanceError> {
        Ok(())
    }
}
