use crate::*;

use yaml_rust::YamlLoader;

use std::fs;

pub fn load_from_yaml<T: YamlConfig>(path: &str) -> T {
    let data = fs::read_to_string(path).expect("Failed to read file");
    let yaml = YamlLoader::load_from_str(&data).expect("Faield to parse YAML");

    T::from_yaml(yaml)
}

pub fn load_from_file<T: FileConfig>(path: &str) -> Result<T, ()> {
    let p: Vec<&str> = path.split(".").collect();

    match *p.last().unwrap() {
        "yaml" | "yml" => Ok(load_from_yaml(path)),
        _ => Err(())
    }
}