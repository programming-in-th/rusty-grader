use crate::errors::{GraderError, GraderResult};
use crate::utils::load_yaml;
use std::{collections::BTreeMap, path::PathBuf};

#[derive(Default, Debug)]
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
    pub fn from(path: PathBuf) -> GraderResult<Self> {
        let yaml = load_yaml(path);
        Ok(Manifest {
            task_id: yaml["task_id"]
                .as_str()
                .ok_or(GraderError::invalid_value())?
                .to_owned(),
            output_only: yaml["output_only"].as_bool().unwrap_or(false),
            time_limit: yaml["time_limit"].as_f64(),
            memory_limit: yaml["memory_limit"].as_i64().map(|limit| limit as u64),
            limit: yaml["limit"]
                .as_hash()
                .map(|limits| -> GraderResult<BTreeMap<String, (f64, u64)>> {
                    Ok(limits
                        .iter()
                        .map(|(language, limit)| {
                            Ok((
                                language
                                    .as_str()
                                    .ok_or(GraderError::invalid_value())?
                                    .to_owned(),
                                (
                                    limit["time_limit"]
                                        .as_f64()
                                        .ok_or(GraderError::invalid_value())?,
                                    limit["memory_limit"]
                                        .as_i64()
                                        .ok_or(GraderError::invalid_value())?
                                        as u64,
                                ),
                            ))
                        })
                        .collect::<GraderResult<BTreeMap<_, _>>>()?)
                })
                .transpose()?,
            compile_files: yaml["compile_files"]
                .as_hash()
                .map(|compile_files| {
                    compile_files
                        .iter()
                        .map(|(language, files)| -> GraderResult<(String, Vec<String>)> {
                            Ok((
                                language
                                    .as_str()
                                    .ok_or(GraderError::invalid_value())?
                                    .to_owned(),
                                files
                                    .as_vec()
                                    .ok_or(GraderError::invalid_value())?
                                    .iter()
                                    .map(|file| {
                                        Ok(file
                                            .as_str()
                                            .ok_or(GraderError::invalid_value())?
                                            .to_owned())
                                    })
                                    .collect::<GraderResult<Vec<_>>>()?,
                            ))
                        })
                        .collect()
                })
                .transpose()?,
            checker: yaml["checker"].as_str().map(|checker| checker.to_owned()),
            grouper: yaml["grouper"].as_str().map(|grouper| grouper.to_owned()),
            groups: yaml["groups"]
                .as_vec()
                .map(|groups| {
                    groups
                        .iter()
                        .map(|group| {
                            Ok((
                                group["full_score"]
                                    .as_i64()
                                    .ok_or(GraderError::invalid_value())?
                                    as u64,
                                group["tests"]
                                    .as_i64()
                                    .ok_or(GraderError::invalid_value())?
                                    as u64,
                            ))
                        })
                        .collect::<GraderResult<Vec<_>>>()
                })
                .ok_or(GraderError::invalid_value())??,
        })
    }
}
