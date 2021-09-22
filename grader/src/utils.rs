use std::{env, fs, path::PathBuf};
use yaml_rust::{Yaml, YamlLoader};

pub fn get_env(name: &'static str) -> String {
    env::var(name).unwrap()
}

pub fn get_base_path() -> PathBuf {
    env::var("BASE_PATH").map_or(
        env::current_dir()
            .unwrap()
            .parent()
            .unwrap()
            .join("example"),
        PathBuf::from,
    )
}

pub fn load_yaml(path: PathBuf) -> Yaml {
    let file = fs::read_to_string(path).expect("Unable to read yaml file");
    YamlLoader::load_from_str(&file).expect("Unable to parse yaml file")[0].clone()
}

pub fn get_code_extension(language: &str) -> String {
    let config = load_yaml(get_base_path().join("scripts").join("config.yaml"));

    for lang in config["language"].as_vec().unwrap() {
        if Some(language) == lang["id"].as_str() {
            return lang["extension"].as_str().unwrap().to_owned();
        }
    }

    "".to_string()
}

pub fn get_message(status: &str) -> String {
    let config = load_yaml(get_base_path().join("scripts").join("config.yaml"));
    config["message"]
        .as_hash()
        .unwrap()
        .clone()
        .iter()
        .find_map(|(schema, value)| {
            if schema.as_str().unwrap() == status {
                Some(value.as_str().unwrap().to_owned())
            } else {
                None
            }
        })
        .unwrap_or_else(String::new)
}

#[cfg(test)]
pub mod tests {
    use crate::utils::get_env;
    use std::{env, fs, path::PathBuf, process::Command};

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

    pub fn get_example_dir() -> PathBuf {
        PathBuf::from(env::current_dir().unwrap())
            .parent()
            .unwrap()
            .join("example")
    }

    pub fn get_tmp_path() -> PathBuf {
        PathBuf::from(get_env("TEMPORARY_PATH"))
    }

    pub fn compile_cpp(tmp_dir: &PathBuf, prog_file: &PathBuf) {
        Command::new(
            &get_example_dir()
                .join("scripts")
                .join("compile_scripts")
                .join("cpp"),
        )
        .arg(&tmp_dir)
        .arg(&prog_file)
        .output()
        .expect("Unable to compile file");
    }
}
