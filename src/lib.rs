pub mod files;

use yaml_rust::Yaml;

use std::io;

// TODO: add derive macro for Config and FileConfig via a crate named rsconfig-macros

pub trait Config {}

pub trait CommandlineConfig : Config {
    fn from_env_args(args: Vec<String>) -> Self;
}

pub trait YamlConfig : Config {
    fn from_yaml(yaml: Vec<Yaml>) -> Self;
    fn save_yaml(&self, path: &str) -> io::Result<()>;
}

pub trait FileConfig : Config + YamlConfig {}

#[cfg(test)]
mod tests {
    use super::*;
    
    use std::{env, fs, io::Result};

    // our config class that we can expand upon to add different values
    #[derive(Debug)]
    struct TestConfig {
        test: bool
    }

    impl Config for TestConfig {}
    impl FileConfig for TestConfig {}

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

    // path to test.yml (ofc)
    const PATH: &str = "testing\\test.yml";

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
    fn file_test() {
        // loads from yaml; could use files::load_from_file(),
        // but since we already know the filetype, it's better to just do this

        let mut config: TestConfig = files::load_from_yaml(PATH);

        println!("{:?}", config);

        change_config(&mut config);
    }

    // swaps the `test` variable value and saves
    fn change_config(config: &mut TestConfig) {
        config.test = !config.test;

        config.save_yaml(PATH).expect("Unable to save");
    }
}
