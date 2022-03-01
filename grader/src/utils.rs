use crate::s;
use std::{env, fs, path::PathBuf};
use yaml_rust::{Yaml, YamlLoader};

pub fn get_env(name: &'static str) -> String {
    env::var(name).unwrap()
}

pub fn get_base_path() -> PathBuf {
    PathBuf::from(env::var("BASE_PATH").unwrap())
}

pub fn load_yaml(path: PathBuf) -> Yaml {
    let file = fs::read_to_string(path).expect("Unable to read yaml file");
    YamlLoader::load_from_str(&file)
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
}

fn yaml_unwrap_hash(yaml: Yaml, arg: &str) -> Option<Yaml> {
    yaml.into_hash().unwrap().remove(&Yaml::String(s!(arg)))
}

pub fn get_code_extension(language: &str) -> String {
    let config = load_yaml(get_base_path().join("scripts").join("config.yaml"));

    for lang in yaml_unwrap_hash(config, "language")
        .unwrap()
        .into_vec()
        .unwrap()
    {
        if Some(language) == lang["id"].as_str() {
            return yaml_unwrap_hash(lang, "extension")
                .unwrap()
                .into_string()
                .unwrap();
        }
    }

    String::new()
}

pub fn get_message(status: &str) -> String {
    let config = load_yaml(get_base_path().join("scripts").join("config.yaml"));
    yaml_unwrap_hash(yaml_unwrap_hash(config, "message").unwrap(), status)
        .map_or(String::new(), |value| value.into_string().unwrap())
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
