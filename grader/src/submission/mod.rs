use crate::utils::{get_env, load_yaml};
use std::{fs, io::Write, path::PathBuf};
use yaml_rust::Yaml;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct Submission {
    pub task_id: String,
    pub submission_id: String,
    pub language: String,
    pub task_manifest: Yaml,
    pub tmp_path: PathBuf,
    pub task_path: PathBuf,
}

impl Default for Submission {
    fn default() -> Self {
        Self {
            task_id: Default::default(),
            submission_id: Default::default(),
            language: Default::default(),
            task_manifest: Yaml::Null,
            tmp_path: Default::default(),
            task_path: Default::default(),
        }
    }
}

impl Submission {
    pub fn new(
        task_id: String,
        submission_id: String,
        language: String,
        code: Vec<String>,
    ) -> Self {
        let mut submission: Submission = Default::default();

        submission.task_id = task_id;
        submission.submission_id = submission_id;
        submission.language = language;

        submission.tmp_path =
            PathBuf::from(get_env("TEMPORARY_PATH")).join(&submission.submission_id);
        fs::create_dir(&submission.tmp_path).expect("Failed to create temporary");

        for (idx, code) in code.iter().enumerate() {
            let mut file = fs::File::create(
                submission
                    .tmp_path
                    .join(String::from("code_") + &idx.to_string()),
            )
            .unwrap();
            file.write(code.as_bytes()).expect("Error writing code");
        }

        submission.task_path = PathBuf::from(get_env("BASE_PATH"))
            .join("tasks")
            .join(&submission.task_id);
        submission.task_manifest = load_yaml(submission.task_path.join("manifest.yaml"));
        // let entries = fs::read_dir(self.task_path.join("compile_files"))?;
        // for entry in entries {
        //     let path = entry?;
        //     fs::copy(&path.path(), self.tmp_path.join(&path.file_name()))?;
        // }

        submission
    }

    pub fn get_manifest(&self) -> Yaml {
        self.task_manifest.clone()
    }
}

impl Drop for Submission {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.tmp_path).expect("Unable to remove submission folder.");
    }
}
