use std::{env, fs, path::PathBuf};
use yaml_rust::{Yaml, YamlLoader};

pub fn get_env(name: &'static str) -> String {
    env::var(name).unwrap()
}

pub fn load_yaml(path: PathBuf) -> Yaml {
    let file = fs::read_to_string(path).expect("Unable to read yaml file");
    YamlLoader::load_from_str(&file).expect("Unable to parse yaml file")[0].clone()
}
