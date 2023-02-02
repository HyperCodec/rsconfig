# RSCONFIG
[<img alt="github" src="https://img.shields.io/github/last-commit/hypercodec/rsconfig" height="20">](https://github.com/hypercodec/rsconfig)
[<img alt="crates.io" src="https://img.shields.io/crates/d/rsconfig" height="20">](https://crates.io/crates/rsconfig)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/rsconfig" height="20">](https://docs.rs/rsconfig)

Simple configuration library to help programs manage their config.

## Importing
If just using RSCONFIG, you can do this:
```toml
[dependencies]
rsconfig = "0.1.3" # replace with latest version
```

## Examples
### CommandlineConfig
```rust
use rsconfig::CommandlineConfig;

use std::env;

// our config class that we can expand upon to add different values
// to expand upon it, simply add more fields and update the import function(s)
#[derive(Debug)]
struct TestConfig {
    test: bool
}

impl CommandlineConfig for TestConfig {
    fn from_env_args(args: Vec<String>) -> Self {
        // check if commandline args contains --test
        Self { test: args.contains(&"--test".to_string()) }
    }
}


fn main() {
    // fetch commandline args
    let args: Vec<String> = env::args().collect();

    // load config from commandline args
    let mut config = TestConfig::from_env_args(args);

    // should output TestConfig { test: true } if --test is in the command
    // otherwise, it will print TestConfig { test: false }
    println!("{:?}", config);

    // you can change the value of the config
    config.test = !config.test;
}
```

### YamlConfig
```rust
// import YamlConfig and files
// files has some useful load functions
use rsconfig::YamlConfig;
use rsconfig::files;

use yaml_rust;

use std::fs;

// our config class that we can expand upon to add different values
// to expand upon it, simply add more fields and update the import function(s)
#[derive(Debug)]
struct TestConfig {
    test: bool
}

impl YamlConfig for TestConfig {
    fn from_yaml(yaml: Vec<yaml_rust::Yaml>) -> Self {
        // fetch "test" value of the first yaml document using yaml_rust crate
        // NOTE: this code is not error-safe, will panic if the correct file formatting is not used
        Self { test: *&yaml[0]["test"].as_bool().unwrap() }
    }

    fn save_yaml(&self, path: &str) -> Result<()> {

        // might want to do this differently for config with more fields
        let mut data = "test: ".to_string();

        // add the value to the file data
        data.push_str(self.test.to_string().as_str());

        // write to the file
        fs::write(path, data).unwrap();

        // return an Ok result
        // required because fs::write could fail, which would pass on an Err(()).
        Ok(())
    }
}


fn main() {
    /*
    NOTE: for a situation where you don't know the filetype,
    you can impl FileConfig and use files::load_from_file, which
    works for multiple different types of files.
    */
    let mut config: TestConfig = files::load_from_yaml();

    // should output TestConfig { test: true } if test: true in the yml file
    // otherwise, it will print TestConfig { test: false }
    println!("{:?}", config);

    // you can change the value of the config
    config.test = !config.test;
}
```

### JsonConfig
```rust
// import JsonConfig and files
// files has some useful load functions
use rsconfig::JsonConfig;
use rsconfig::files;

use serde_json;

use std::fs;

// our config class that we can expand upon to add different values
// to expand upon it, simply add more fields and update the import function(s)
#[derive(Debug)]
struct TestConfig {
    test: bool
}

impl JsonConfig for TestConfig {
    fn from_json(val: serde_json::Value) -> Self {
        // look for "test" val
        // NOTE: this code is not error-safe, will panic if the json does not contain a bool named "test"
        Self { test: val["test"].as_bool().unwrap() }
    }

    fn save_json(&self, path: &str) -> io::Result<()> {
        // convert to json pretty format and save
        let data = serde_json::to_string_pretty(&Value::from(self.test)).unwrap();
        fs::write(path, data).unwrap();

        Ok(())
    }
}

fn main() {
    /*
    NOTE: for a situation where you don't know the filetype,
    you can impl FileConfig and use files::load_from_file, which
    works for multiple different types of files.
    */
    let mut config: TestConfig = files::load_from_json();

    // should output TestConfig { test: true } if {"test": true} in the json file
    // otherwise, it will print TestConfig { test: false }
    println!("{:?}", config);

    // you can change the value of the config
    config.test = !config.test;
}
```

### FileConfig
```rust
#[derive(Debug)]
struct TestConfig {
    test: bool
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
        // convert to json pretty format and save
        let mut m: Hashmap<&str, Value> = Hashmap::new();
        m.insert("test", &Value::from(self.test));
        let data = serde_json::to_string_pretty(m).unwrap();
        fs::write(path, data).unwrap();

        Ok(())
    }
}

impl FileConfig for TestConfig {}
```


## License
Licensed under either of Apache License, Version 2.0 or MIT license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
