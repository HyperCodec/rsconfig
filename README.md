# RSCONFIG
Simple configuration library to help programs manage their config.

## Importing
If just using RSCONFIG, you can do this:
```toml
[dependencies]
rsconfig = "0.1.0" # replace with latest version
```

Using YamlConfig, you will have to add [yaml-rust](https://crates.io/crates/yaml-rust):
```toml
[dependencies]
rsconfig = "0.1.0" # replace with latest version
yaml-rust = "0.4.0" # replace with latest version used by RSCONFIG
```

## Examples
### CommandlineConfig
```rust
use rsconfig::CommandlineConfig;
use std::{env, fs, io::Result};

// our config class that we can expand upon to add different values
// to expand upon it, simply add more fields and update the import function(s)
#[derive(Debug)]
struct TestConfig {
    test: bool
}

impl CommandlineConfig for TestConfig {
    fn from_env_args(args: Vec<String>) -> Self {
        // check if commandline args contains --test
        Self { test: args.contains(&"test".to_string()) }
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
// files has 
use rsconfig::YamlConfig;
use rsconfig::files;

// our config class that we can expand upon to add different values
// to expand upon it, simply add more fields and update the import function(s)
#[derive(Debug)]
struct TestConfig {
    test: bool
}

impl YamlConfig for TestConfig {
    fn from_yaml(yaml: Vec<yaml_rust::Yaml>) -> Self {
        // fetch "test" value of first yaml document using yaml_rust crate
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
    you can impl FileConfig and use files::load_from_file, which would
    work for different types of files.
    */
    let mut config: TestConfig = files::load_from_yaml();

    // should output TestConfig { test: true } if test: true in the yml file
    // otherwise, it will print TestConfig { test: false }
    println!("{:?}", config);

    // you can change the value of the config
    config.test = !config.test;
}
```