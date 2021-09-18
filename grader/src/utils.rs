use std::{env, fs, path::PathBuf};
use yaml_rust::{Yaml, YamlLoader};

pub fn get_env(name: &'static str) -> String {
    env::var(name).unwrap()
}

pub fn load_yaml(path: PathBuf) -> Yaml {
    let file = fs::read_to_string(path).expect("Unable to read yaml file");
    YamlLoader::load_from_str(&file).expect("Unable to parse yaml file")[0].clone()
}

#[cfg(test)]
pub mod tests {
    use crate::utils::get_env;
    use std::{fs, path::PathBuf, process::Command};

    pub struct TempDir(pub PathBuf);

    impl Drop for TempDir {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).expect("Unable to remove tmp directory");
        }
    }

    impl TempDir {
        pub fn new(tmp_name: &'static str) -> Self {
            let tmp_path = PathBuf::from(get_env("TEMPORARY_PATH")).join(tmp_name);
            fs::create_dir(&tmp_path).expect("Unable to create tmp directory");
            Self(tmp_path)
        }
    }

    pub fn get_base_dir(ext: &'static str) -> PathBuf {
        PathBuf::from(get_env("BASE_PATH")).join(ext)
    }

    pub fn get_tmp_path() -> PathBuf {
        PathBuf::from(get_env("TEMPORARY_PATH"))
    }

    pub fn compile_cpp(tmp_dir: &PathBuf, prog_file: &PathBuf) {
        Command::new(&get_base_dir("scripts").join("compile_scripts").join("cpp"))
            .arg(&tmp_dir)
            .arg(&prog_file)
            .output()
            .expect("Unable to compile file");
    }
}
