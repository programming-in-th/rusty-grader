use crate::utils::load_yaml;
use std::{collections::BTreeMap, path::PathBuf};

#[derive(Default, Debug, Clone)]
pub struct Manifest {
    pub task_id: String,
    pub output_only: bool,
    pub time_limit: Option<f64>,
    pub memory_limit: Option<u64>,
    pub limit: Option<BTreeMap<String, (f64, u64)>>,
    pub compile_files: Option<BTreeMap<String, Vec<String>>>,
    pub checker: Option<String>,
    pub grouper: Option<String>,
    pub groups: Vec<(u64, u64)>,
}

impl Manifest {
    pub fn from(path: PathBuf) -> Self {
        let yaml = load_yaml(path);
        let mut manifest: Manifest = Default::default();

        manifest.task_id = yaml["task_id"].as_str().unwrap().to_owned();
        manifest.output_only = yaml["output_only"].as_bool().unwrap_or(false);
        manifest.time_limit = yaml["time_limit"].as_f64();
        manifest.memory_limit = yaml["memory_limit"].as_i64().map(|limit| limit as u64);

        if let Some(limits) = yaml["limit"].as_hash() {
            manifest.limit = Some(
                limits
                    .clone()
                    .iter()
                    .map(|(language, limit)| {
                        (
                            language.as_str().unwrap().to_owned(),
                            (
                                limit["time_limit"].as_f64().unwrap(),
                                limit["memory_limit"].as_i64().unwrap() as u64,
                            ),
                        )
                    })
                    .collect(),
            );
        }

        if let Some(compile_files) = yaml["compile_files"].as_hash() {
            manifest.compile_files = Some(
                compile_files
                    .clone()
                    .iter()
                    .map(|(language, files)| {
                        (
                            language.as_str().unwrap().to_owned(),
                            files
                                .as_vec()
                                .unwrap()
                                .clone()
                                .iter()
                                .map(|file| file.as_str().unwrap().to_owned())
                                .collect(),
                        )
                    })
                    .collect(),
            );
        }

        manifest.checker = yaml["checker"].as_str().map(|checker| checker.to_owned());
        manifest.grouper = yaml["grouper"].as_str().map(|grouper| grouper.to_owned());

        for group in yaml["groups"].as_vec().unwrap() {
            manifest.groups.push((
                group["full_score"].as_i64().unwrap() as u64,
                group["tests"].as_i64().unwrap() as u64,
            ));
        }

        manifest
    }
}