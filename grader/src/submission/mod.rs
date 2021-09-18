extern crate yaml_rust;

use crate::utils::{get_env, load_yaml};
use std::{fs, io, io::Write, path::PathBuf};
use yaml_rust::Yaml;

pub struct Submission {
    pub task_id: String,
    pub submission_id: String,
    pub language: String,
    pub code: Vec<String>,
    pub task_manifest: Yaml,
    pub tmp_path: PathBuf,
    pub task_path: PathBuf,
}

impl Submission {
    pub fn init(&mut self) -> io::Result<()> {
        self.tmp_path = PathBuf::from(get_env("TEMPORARY_PATH")).join(&self.submission_id);
        fs::create_dir(&self.tmp_path)?;

        for (idx, code) in self.code.iter().enumerate() {
            let mut file =
                fs::File::create(self.tmp_path.join(String::from("code_") + &idx.to_string()))?;
            file.write(code.as_bytes())?;
        }

        self.task_path = PathBuf::from(get_env("BASE_PATH"))
            .join("tasks")
            .join(&self.task_id);
            
        self.task_manifest = load_yaml(self.task_path.join("manifest.yaml"));
        
        let entries = fs::read_dir(self.task_path.join("compile_files"))?;
        for entry in entries {
            let path = entry?;
            fs::copy(&path.path(), self.tmp_path.join(&path.file_name()))?;
        }

        Ok(())
    }
}

impl Drop for Submission {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.tmp_path).expect("Unable to remove submission folder.");
    }
}
