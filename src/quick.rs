use crate::*;

/// Creates a list of commandline flags that were used in command arguments
/// Command line flags are any arguments that start with `--` and do not contain `:`
/// Useful for arbitrary options when you just want to quickstart a project instead of creating your own CommandlineConfig
pub struct FlagConfig(Vec<String>);

impl CommandlineConfig for FlagConfig {
    fn from_env_args(args: Vec<String>) -> Self {
        let mut flags = Vec::new();

        for arg in args {
            if arg.starts_with("--") && !arg.contains(":") {
                flags.push(arg);
            }
        }

        Self(flags)
    }
}