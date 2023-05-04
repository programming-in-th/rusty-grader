use crate::combine_argument;
use crate::errors::{GraderError, GraderResult};
use crate::utils::get_env;
use std::path::PathBuf;
use tokio::{fs, process::Command};
use std::str::FromStr;

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
    fn get_run_arguments(&self) -> GraderResult<Vec<String>> {
        Ok(combine_argument![
            "-b",
            self.box_id.to_string(),
            "-M",
            self.log_file
                .to_str()
                .ok_or(GraderError::invalid_to_str())?
                .to_string(),
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
            format!("--cg-mem={}", self.memory_limit),
            format!("--dir={}", get_env("ALTERNATIVE_PATH")),
            "--run",
            "--",
            "runner"
        ])
    }

    pub async fn get_result(&self) -> GraderResult<InstanceResult> {
        let log_content = fs::read_to_string(&self.log_file).await?;
        let mut result: InstanceResult = Default::default();
        let mut memory_limit_exceeded = false;
        for log_line in log_content.lines() {
            let args: Vec<&str> = log_line.split(':').collect();
            log::info!("{args:?}");
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
                    "time" => result.time_usage = args[1].parse()?,
                    "cg-mem" => result.memory_usage = args[1].parse()?,
                    "cg-oom-killed" => memory_limit_exceeded = args[1].trim() == "1",
                    _ => (),
                }
            }
        }
        if memory_limit_exceeded
            || result.memory_usage >= self.memory_limit && result.status == Default::default()
        {
            result.status = RunVerdict::VerdictMLE;
        }
        Ok(result)
    }

    pub async fn init(&mut self) -> GraderResult<()> {
        let tmp_path = get_env("TEMPORARY_PATH");

        // <PathBuf as FromStr>::Error is Infalliable
        let box_path = PathBuf::from_str(&tmp_path).unwrap().join(format!("{}", self.box_id));

        let box_path = Command::new(get_env("ISOLATE_PATH"))
            .args(&["--init", "--cg", "-b"])
            .arg(format!("{}", self.box_id))
            .output()
            .await?;

        let box_path = dbg!(String::from_utf8(box_path.stdout)?);
        self.box_path = PathBuf::from(box_path.trim_end_matches('\n')).join("box");

        self.log_file = PathBuf::from(tmp_path).join(format!("tmp_log_{}.txt", self.box_id));

        fs::copy(&self.input_path, &self.box_path.join("input")).await?;

        fs::copy(
            &self.bin_path,
            &self.box_path.join(
                self.bin_path
                    .file_name()
                    .ok_or(GraderError::invalid_to_str())?,
            ),
        )
        .await?;

        fs::copy(&self.runner_path, &self.box_path.join("runner")).await?;
        Ok(())
    }

    pub async fn run(&self) -> GraderResult<InstanceResult> {
        let args = self.get_run_arguments()?;
        Command::new(get_env("ISOLATE_PATH"))
            .args(args)
            .output()
            .await?;

        let result = self.get_result().await?;
        if result.status == RunVerdict::VerdictOK {
            fs::copy(&self.box_path.join("output"), &self.output_path).await?;
        }
        Ok(result)
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        std::process::Command::new(get_env("ISOLATE_PATH"))
            .args(&["--cleanup", "--cg", "-b"])
            .arg(self.box_id.to_string())
            .output()
            .ok();

        if self.log_file.is_file() {
            std::fs::remove_file(&self.log_file).ok();
        }
    }
}
