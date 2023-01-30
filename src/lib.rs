pub mod files;

use serde_json::Value;
use yaml_rust::Yaml;

use std::io;

pub trait CommandlineConfig {
    fn from_env_args(args: Vec<String>) -> Self;
}

pub trait YamlConfig {
    fn from_yaml(yaml: Vec<Yaml>) -> Self;
    fn save_yaml(&self, path: &str) -> io::Result<()>;
}

pub trait JsonConfig {
    fn from_json(val: Value) -> Self;
    fn save_json(&self, path: &str) -> io::Result<()>;
}

// TODO: add more file types and derive macro
pub trait FileConfig : YamlConfig + JsonConfig {}

#[cfg(test)]
mod tests {
    use super::*;
    
    use std::{env, fs, io::Result};

    // config class that we can expand upon to add different values
    #[derive(Debug)]
    struct TestConfig {
        test: bool
    }

    impl CommandlineConfig for TestConfig {
        fn from_env_args(args: Vec<String>) -> Self {
            Self { test: args.contains(&"test".to_string()) }
        }
    }

    impl YamlConfig for TestConfig {
        fn from_yaml(yaml: Vec<yaml_rust::Yaml>) -> Self {
            Self { test: *&yaml[0]["test"].as_bool().unwrap() }
        }

        fn save_yaml(&self, path: &str) -> Result<()> {
            let mut data = "test: ".to_string();
            data.push_str(self.test.to_string().as_str());

            fs::write(path, data).unwrap();
    
            Ok(())
        }
    }

    impl JsonConfig for TestConfig {
        fn from_json(val: Value) -> Self {
            Self { test: val["test"].as_bool().unwrap() }
        }

        fn save_json(&self, path: &str) -> io::Result<()> {
            fs::write(path, serde_json::to_string_pretty(&Value::from(self.test)).unwrap()).unwrap();

            Ok(())
        }
    }

    impl FileConfig for TestConfig {}

    // path to test files
    const YAML_PATH: &str = "testing\\test.yml";
    const JSON_PATH: &str = "testing\\test.json";

    #[test]
    fn args_test() {
        // under normal test command (cargo test --package rsconfig --lib -- tests --nocapture),
        // this will always create `config` with `test` as false

        let args: Vec<String> = env::args().collect();

        let mut config = TestConfig::from_env_args(args);

        println!("{:?}", config);

        change_config(&mut config);
    }

    #[test]
    fn yaml_test() {
        // loads from yaml; could use files::load_from_file(),
        // but since we already know the filetype, it's better to just do this

        let mut config: TestConfig = files::load_from_yaml(YAML_PATH);

        println!("{:?}", config);

        change_config(&mut config);
    }

    #[test]
    fn json_test() {
        // loads from json; could use files::load_from_file(),
        // but since we already know the filetype, it's better to just do this

        let mut config: TestConfig = files::load_from_json(JSON_PATH);

        println!("{:?}", config);

        change_config(&mut config);
        
        // saving both yaml and json but idc don't want to copy one line of code
        config.save_json(JSON_PATH).expect("Unable to save");
    }

    // swaps the `test` variable value and saves
    fn change_config(config: &mut TestConfig) {
        config.test = !config.test;

        config.save_yaml(YAML_PATH).expect("Unable to save");

        println!("{:?}", config);
    }
}
