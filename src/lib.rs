#![warn(missing_docs)]

//! A simple configuration library that allows developers to quickly make configuration for their apps.

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
/// }
/// ```
pub trait CommandlineConfig {
    /// Initialize a CommandlineConfig struct given the commandline arguments that the program was run with.
    /// ### Example
    /// ```rust
    /// # use rsconfig::CommandlineConfig;
    /// # struct T { test: bool }
    /// # impl CommandlineConfig for T {
    /// fn from_env_args(args: Vec<String>) -> Self {
    ///     // check if commandline args contains --test
    ///     Self { test: args.contains(&"--test".to_string()) }
    /// }
    /// # }
    /// ```
    fn from_env_args(args: Vec<String>) -> Self;
}

/// Represents a configuration struct that can be created from a YAML (YML) file.
/// ### Example
/// ```rust
/// use yaml_rust;
/// use rsconfig::YamlConfig;
/// 
/// use std::{fs, io::Result};
///
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
    /// # use yaml_rust;
    /// # use rsconfig::YamlConfig;
    /// # use std::io::Result;
    /// 
    /// # struct T { test: bool }
    /// # impl YamlConfig for T {
    /// fn from_yaml(yaml: Vec<yaml_rust::Yaml>) -> Self {
    ///     // fetch "test" value of the first yaml document using yaml_rust crate
    ///     // NOTE: this code is not error-safe, will panic if the file does not contain a bool named "test"
    ///     Self { test: *&yaml[0]["test"].as_bool().unwrap() }
    /// }
    /// # fn save_yaml(&self, path: &str) -> Result<()> {Ok(())}
    /// # }
    /// ```
    fn from_yaml(yaml: Vec<Yaml>) -> Self;

    /// Save a YamlConfig struct's contents to a YAML (YML) file.
    /// ### Example
    /// ```rust
    /// # use std::{fs, io::Result};
    /// # use rsconfig::YamlConfig;
    /// # use rust_yaml::Yaml;
    /// 
    /// # struct T { test: bool }
    /// # impl YamlConfig for T {
    /// # fn from_yaml(yaml: Vec<Yaml>) -> Self {Self{test: false}}
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
    /// # }
    /// ```
    fn save_yaml(&self, path: &str) -> io::Result<()>;
}

/// Represents a configuration struct that can be created from a JSON file.
/// ### Example
/// ```rust
/// use serde_json;
///
/// use rsconfig::JsonConfig;
/// 
/// use std::fs;
///
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
///         let mut m: Hashmap<&str, Value> = Hashmap::new();
///         m.insert("test", &Value::from(self.test));
///         let data = serde_json::to_string_pretty(m).unwrap();
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
    /// # use serde_json;
    /// # use rsconfig::JsonConfig;
    /// # use std::io::Result;
    /// 
    /// # struct T { test: bool }
    /// # impl JsonConfig for T {
    /// fn from_json(val: serde_json::Value) -> Self {
    ///         // look for "test" val
    ///         // NOTE: this code is not error-safe, will panic if the json does not contain a bool named "test"
    ///         Self { test: val["test"].as_bool().unwrap() }
    /// }
    /// # fn save_json(&self, path: &str) -> Result<()> {Ok(())}
    /// # }
    /// ```
    fn from_json(val: Value) -> Self;

    /// Save a JsonConfig struct's contents to a JSON file.
    /// ### Example
    /// ```rust
    /// # use std::{fs, io::Result, collections::HashMap};
    /// # use serde_json::Value;
    /// # use rsconfig::JsonConfig;
    /// 
    /// # struct T { test: bool }
    /// # impl JsonConfig for T {
    /// # fn from_json(val: Value) -> Self{Self{test: true}}
    /// fn save_json(&self, path: &str) -> Result<()> {
    ///         // convert to json pretty format and save
    ///         let mut m: HashMap<&str, Value> = HashMap::new();
    ///         m.insert("test", Value::from(self.test));
    ///         let data = serde_json::to_string_pretty(&m).unwrap();
    ///         fs::write(path, data).unwrap();
    ///
    ///         Ok(())
    /// }
    /// # }
    /// ```
    fn save_json(&self, path: &str) -> io::Result<()>;
}

/// Represents a configuration struct that can be created from a number of file types.
/// ### Example
/// ```rust
/// use rsconfig::{YamlConfig, JsonConfig};
/// use rsconfig_macros::FileConfig
///
/// use serde_json;
/// use yaml_rust;
/// 
/// // rsconfig-macros crate has a derive macro for this trait
/// #[derive(Debug, FileConfig)]
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
///         // convert to json pretty format and save
///         let mut m: Hashmap<&str, Value> = Hashmap::new();
///         m.insert("test", &Value::from(self.test));
///         let data = serde_json::to_string_pretty(m).unwrap();
///         fs::write(path, data).unwrap();
///
///         Ok(())
///     }
/// }
/// ```

pub trait FileConfig: YamlConfig + JsonConfig {}

#[cfg(test)]
mod tests {
    use super::*;
    use rsconfig_macros::*;

    use std::{collections::HashMap, env, fs, io::Result};

    // config class that we can expand upon to add different values
    #[derive(Debug, FileConfig)]
    struct TestConfig {
        test: bool,
    }

    impl CommandlineConfig for TestConfig {
        fn from_env_args(args: Vec<String>) -> Self {
            Self {
                test: args.contains(&"test".to_string()),
            }
        }
    }

    impl YamlConfig for TestConfig {
        fn from_yaml(yaml: Vec<yaml_rust::Yaml>) -> Self {
            Self {
                test: *&yaml[0]["test"].as_bool().unwrap(),
            }
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
            Self {
                test: val["test"].as_bool().unwrap(),
            }
        }

        fn save_json(&self, path: &str) -> io::Result<()> {
            // convert to json pretty format and save
            let mut m: HashMap<&str, Value> = HashMap::new();
            m.insert("test", Value::from(self.test));
            let data = serde_json::to_string_pretty(&m).unwrap();
            fs::write(path, data).unwrap();

            Ok(())
        }
    }

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

    #[test]
    fn file_test() {
        let mut config: TestConfig =
            files::load_from_file(YAML_PATH).expect("Unable to load from file");

        println!("{:?}", config);

        change_config(&mut config);
    }

    // swaps the `test` variable value and saves
    fn change_config(config: &mut TestConfig) {
        config.test = !config.test;

        config.save_yaml(YAML_PATH).expect("Unable to save");

        println!("{:?}", config);
    }
}
