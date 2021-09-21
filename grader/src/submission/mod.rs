use crate::utils::{get_base_path, get_code_extension, get_env};
use manifest::Manifest;
use std::{
    fs, io,
    io::{Error, Write},
    path::PathBuf,
    process::Command,
};

pub mod manifest;

#[cfg(test)]
mod tests;

#[derive(Default, Debug, Clone)]
pub struct Submission {
    pub task_id: String,
    pub submission_id: String,
    pub language: String,
    pub code: Vec<String>,
    pub code_path: Vec<PathBuf>,
    pub task_manifest: Manifest,
    pub tmp_path: PathBuf,
    pub task_path: PathBuf,
    pub bin_path: PathBuf,
}

impl Submission {
    pub fn init(&mut self) -> io::Result<()> {
        self.tmp_path = PathBuf::from(get_env("TEMPORARY_PATH")).join(&self.submission_id);
        fs::create_dir(&self.tmp_path)?;

        let extension = get_code_extension(&self.language);
        for (idx, code) in self.code.iter().enumerate() {
            let code_path = self
                .tmp_path
                .join(format!("code_{}.{}", &idx.to_string(), &extension));
            let mut file = fs::File::create(&code_path)?;
            file.write(code.as_bytes())?;

            self.code_path.push(code_path.clone());
        }

        self.task_path = get_base_path().join("tasks").join(&self.task_id);
        self.task_manifest = Manifest::from(self.task_path.join("manifest.yaml"));

        if self.task_path.join("compile_files").is_dir() {
            let entries = fs::read_dir(self.task_path.join("compile_files"))?;
            for entry in entries {
                let path = entry?;
                fs::copy(&path.path(), self.tmp_path.join(&path.file_name()))?;
            }
        }
        Ok(())
    }

    pub fn compile(&mut self) -> io::Result<()> {
        let compiler_path = get_base_path()
            .join("scripts")
            .join("compile_scripts")
            .join(&self.language);

        let mut args = vec![self.tmp_path.clone()];
        args = args
            .iter()
            .cloned()
            .chain(self.code_path.iter().cloned())
            .collect();

        if let Some(compile_files) = &self.task_manifest.compile_files {
            for compile_file in compile_files.get(&self.language).unwrap() {
                args.push(self.task_path.join(&compile_file));
            }
        }

        let compile_output = Command::new(compiler_path).args(args).output()?;
        let compile_output_args = String::from_utf8(compile_output.stdout)
            .unwrap()
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let return_code: i32 = compile_output_args.get(0).unwrap().parse().unwrap();

        if return_code != 0 {
            return Err(Error::from_raw_os_error(return_code));
        }

        self.bin_path = PathBuf::from(compile_output_args.get(1).unwrap());
        Ok(())
    }
}

impl Drop for Submission {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.tmp_path).expect("Unable to remove submission folder.");
    }
}
