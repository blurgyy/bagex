use std::path::PathBuf;

use color_eyre::Report;
use serde::Deserialize;

#[derive(Default, Clone, Deserialize, Debug)]
pub struct BagexConfig {
    pub path: Option<Vec<std::path::PathBuf>>,
    pub env: Option<toml::Value>,
    pub exe: Option<toml::Value>,
    pub clear_env: Option<bool>,
}

impl BagexConfig {
    pub fn validate(self: BagexConfig) -> bool {
        // Validate section `env`
        if let toml::Value::Table(env_table) = self.env.unwrap() {
            for (env_name, info) in env_table {
                if let toml::Value::Table(value_exe_pairs) = info {
                    for (_value, exes) in value_exe_pairs {
                        if let toml::Value::Array(_exes) = exes {
                            continue;
                        } else {
                            // This happens when floating number is not quoted as string, e.g.:
                            //
                            //      [env.pi]
                            //      3.1415926 = ["printenv"]
                            //
                            // In this case, toml will interprete the 3.14 as two tables "3" and
                            // its descendant "1415926".  To include character '.' in an
                            // environment variable's value, surround it with single or double quotes:
                            //
                            //      [env.pi]
                            //      "3.1415926" = ["printenv"]
                            //
                            log::error!("Value of variable {} is expanded as a toml table", env_name);
                            log::error!(
                                "because it contains character '.' (dot)."
                            );
                            log::error!("Quote the value to fix this");
                            return false;
                        }
                    }
                }
            }
        }

        // Validate section `exe`

        // There is not much to validate for `exe`.

        true
    }

    pub fn from_pathbuf(path: PathBuf) -> Result<BagexConfig, Report> {
        let confstr = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&confstr)?)
    }
}

// Author: Blurgy <gy@blurgy.xyz>
// Date:   Jul 25 2021, 23:50 [CST]
