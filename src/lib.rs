#![warn(missing_docs)]

//! A simple configuration library that allows for easy config.

/// Contains useful functions for importing from files
pub mod files;

use serde_json::Value;
use yaml_rust::Yaml;

use std::io;


/// Represents a configuration struct that can be created from commandline arguments.
/// ### Example Code
/// ```rust
/// use rsconfig::CommandlineConfig;
///
/// use std::env;
///
/// // our config class that we can expand upon to add different values
/// // to expand upon it, simply add more fields and update the import function(s)
/// #[derive(Debug)]
/// struct TestConfig {
///     test: bool
/// }
/// 
/// impl CommandlineConfig for TestConfig {
///     fn from_env_args(args: Vec<String>) -> Self {
///         // check if commandline args contains --test
///         Self { test: args.contains(&"--test".to_string()) }
///     }
/// }
/// 
/// 
/// fn main() {
///     // fetch commandline args
///     let args: Vec<String> = env::args().collect();
/// 
///     // load config from commandline args
///     let mut config = TestConfig::from_env_args(args);
/// 
///     // should output TestConfig { test: true } if --test is in the command
///     // otherwise, it will print TestConfig { test: false }
///     println!("{:?}", config);
/// 
///     // you can change the value of the config
///     config.test = !config.test;
/// }
/// ```
pub trait CommandlineConfig {
    /// Initialize a CommandlineConfig struct given the commandline arguments that the program was run with.
    /// ### Example
    /// ```rust
    /// fn from_env_args(args: Vec<String>) -> Self {
    ///     // check if commandline args contains --test
    ///     Self { test: args.contains(&"--test".to_string()) }
    /// }
    /// ```
    fn from_env_args(args: Vec<String>) -> Self;
}

/// Represents a configuration struct that can be created from a YAML (YML) file.
/// ### Example
/// ```rust
/// struct TestConfig {
///     test: bool
/// }
/// 
/// impl YamlConfig for TestConfig {
///     fn from_yaml(yaml: Vec<yaml_rust::Yaml>) -> Self {
///         // fetch "test" value of the first yaml document using yaml_rust crate
///         // NOTE: this code is not error-safe, will panic if the correct file formatting is not used
///         Self { test: *&yaml[0]["test"].as_bool().unwrap() }
///     }
/// 
///     fn save_yaml(&self, path: &str) -> Result<()> {
///         // might want to do this differently for config with more fields
///         let mut data = "test: ".to_string();
/// 
///         // add the value to the file data
///         data.push_str(self.test.to_string().as_str());
/// 
///         // write to the file
///         fs::write(path, data).unwrap();
/// 
///         // return an Ok result
///         // required because fs::write could fail, which would pass on an Err(()).
///         Ok(())
///     }
/// }
/// ```
pub trait YamlConfig {
    /// Initialize a YamlConfig struct given a list of Yaml documents from a parsed file.
    /// ### Example
    /// ```rust
    /// fn from_yaml(yaml: Vec<yaml_rust::Yaml>) -> Self {
    ///         // fetch "test" value of the first yaml document using yaml_rust crate
    ///         // NOTE: this code is not error-safe, will panic if the file does not contain a bool named "test"
    ///         Self { test: *&yaml[0]["test"].as_bool().unwrap() }
    ///     }
    /// ```
    fn from_yaml(yaml: Vec<Yaml>) -> Self;

    /// Save a YamlConfig struct's contents to a YAML (YML) file.
    /// ### Example
    /// ```rust
    /// fn save_yaml(&self, path: &str) -> Result<()> {
    ///         // might want to do this differently for config with more fields
    /// 
    ///         let mut data = "test: ".to_string();
    /// 
    ///         // add the value to the file data
    ///         data.push_str(self.test.to_string().as_str());
    /// 
    ///         // write to the file
    ///         fs::write(path, data).unwrap();
    /// 
    ///         // return an Ok result
    ///         // required because fs::write could fail, which would pass on an Err(()).
    ///         Ok(())
    ///     }
    /// ```
    fn save_yaml(&self, path: &str) -> io::Result<()>;
}

/// Represents a configuration struct that can be created from a JSON file.
/// ### Example
/// ```rust
/// #[derive(Debug)]
/// struct TestConfig {
///     test: bool
/// }
/// 
/// impl JsonConfig for TestConfig {
///     fn from_json(val: serde_json::Value) -> Self {
///         // look for "test" val
///         // NOTE: this code is not error-safe, will panic if the json does not contain a bool named "test"
///         Self { test: val["test"].as_bool().unwrap() }
///     }
/// 
///     fn save_json(&self, path: &str) -> io::Result<()> {
///         // convert to json pretty format and save
///         let data = serde_json::to_string_pretty(&Value::from(self.test)).unwrap();
///         fs::write(path, data).unwrap();
/// 
///         Ok(())
///     }
/// }
/// ```
pub trait JsonConfig {
    /// Initialize a JsonConfig struct from a given json value.
    /// ### Example
    /// ```rust
    /// fn from_json(val: serde_json::Value) -> Self {
    ///         // look for "test" val
    ///         // NOTE: this code is not error-safe, will panic if the json does not contain a bool named "test"
    ///         Self { test: val["test"].as_bool().unwrap() }
    ///     }
    /// ```
    fn from_json(val: Value) -> Self;

    /// Save a JsonConfig struct's contents to a JSON file.
    /// ### Example
    /// ```rust
    /// fn save_json(&self, path: &str) -> io::Result<()> {
    ///         // convert to json pretty format and save
    ///         let data = serde_json::to_string_pretty(&Value::from(self.test)).unwrap();
    ///         fs::write(path, data).unwrap();
    /// 
    ///         Ok(())
    ///     }
    /// ```
    fn save_json(&self, path: &str) -> io::Result<()>;
}

/// Represents a configuration struct that can be created from a number of file types.
/// ### Example
/// ```rust
/// #[derive(Debug)]
/// struct TestConfig {
///     test: bool
/// }
/// 
/// impl YamlConfig for TestConfig {
///     fn from_yaml(yaml: Vec<yaml_rust::Yaml>) -> Self {
///         Self { test: *&yaml[0]["test"].as_bool().unwrap() }
///     }
/// 
///     fn save_yaml(&self, path: &str) -> Result<()> {
///         let mut data = "test: ".to_string();
///         data.push_str(self.test.to_string().as_str());
/// 
///         fs::write(path, data).unwrap();
/// 
///         Ok(())
///     }
/// }
/// 
/// impl JsonConfig for TestConfig {
///     fn from_json(val: Value) -> Self {
///         Self { test: val["test"].as_bool().unwrap() }
///     }
/// 
///     fn save_json(&self, path: &str) -> io::Result<()> {
///         fs::write(path, serde_json::to_string_pretty(&Value::from(self.test)).unwrap()).unwrap();
/// 
///         Ok(())
///     }
/// }
/// 
/// impl FileConfig for TestConfig {}
/// ```

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
