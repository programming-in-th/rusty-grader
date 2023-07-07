use crate::combine_argument;
use crate::errors::{GraderError, GraderResult};
use crate::utils::get_env;
use std::path::PathBuf;
use tokio::{fs, process::Command};
use anyhow::Context;

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
                .ok_or(GraderError::invalid_to_str())
                .with_context(|| "Unable to convert OsStr to str")?
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
                match args[0] {
                    "status" => {
                        result.status = match args[1] {
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

        let box_path = Command::new(get_env("ISOLATE_PATH"))
            .args(["--init", "--cg", "-b"])
            .arg(format!("{}", self.box_id))
            .output()
            .await?;

        let box_path = String::from_utf8(box_path.stdout)?;
        self.box_path = PathBuf::from(box_path.trim_end_matches('\n')).join("box");

        self.log_file = PathBuf::from(tmp_path).join(format!("tmp_log_{}.txt", self.box_id));

        let from = self.box_path.as_path();
        let to = self.log_file.as_path();

        fs::copy(&self.input_path, &self.box_path.join("input")).await.with_context(|| format!("Copy from {from:?} to {to:?}"))?;

        let from = self.bin_path.as_path();
        let to = self.box_path.join(
            self.bin_path
                .file_name()
                .ok_or(GraderError::invalid_to_str())
                .with_context(|| "Cannot convert OsStr to str")?,
        );

        fs::copy(
            from,
            to.as_path(),
        )
        .await
        .with_context(|| format!("Copy from {from:?} to {to:?}"))?;

        let from = self.runner_path.as_path();
        let to = self.box_path.join("runner");
        fs::copy(from, to.as_path()).await.with_context(|| "Copy from {from:?} to {to:?}")?;
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
            let from = self.box_path.join("output");
            let to = self.output_path.as_path();

            fs::copy(from, to).await.with_context(|| "Copy from {from:?} to {to:?}")?;
        }
        Ok(result)
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        std::process::Command::new(get_env("ISOLATE_PATH"))
            .args(["--cleanup", "--cg", "-b"])
            .arg(self.box_id.to_string())
            .output()
            .ok();

        if self.log_file.is_file() {
            std::fs::remove_file(&self.log_file).ok();
        }
    }
}
