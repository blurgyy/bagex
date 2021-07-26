use std::{collections::HashMap, path::PathBuf, str::FromStr};

use crate::config::BagexConfig;

fn get_home_dir() -> PathBuf {
    PathBuf::from_str(&std::env::var("HOME").unwrap_or("/".to_string()))
        .unwrap_or_default()
}

/// If the environment variable $XDXDG_CONFIG_HOME is set, returns
/// $XDG_CONFIG_HOME/bages/config.toml, else returns $HOME/.config/bagex/config.toml
pub fn default_config_file() -> PathBuf {
    if let Ok(path) = std::env::var("XDG_CONFIG_HOME") {
        PathBuf::from_str(&path).unwrap()
    } else {
        get_home_dir().join(".config")
    }
    .join("bagex")
    .join("config.toml")
}

pub fn compose_and_set_path(additional_path: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut path = additional_path;
    path = path
        .iter()
        .map(|x| {
            PathBuf::from_str(&shellexpand::tilde(
                &x.to_str().unwrap().to_string(),
            ))
            .unwrap()
        })
        .collect();
    path.extend(
        std::env::var("PATH")
            .unwrap()
            .split(":")
            .map(|x| PathBuf::from_str(x).unwrap())
            .collect::<Vec<PathBuf>>(),
    );
    log::trace!("Paths in composed PATH: {:#?}", path);
    let env_path: String = path
        .iter()
        .map(|x| x.to_str().unwrap().to_string())
        .collect::<Vec<String>>()
        .join(":");
    log::trace!("Composed PATH as environment variable: {:#?}", env_path);
    std::env::set_var("PATH", env_path);

    path
}

pub fn get_executable_path(
    req_exe_name: String,
    path: Vec<PathBuf>,
) -> PathBuf {
    let mut exe: PathBuf = PathBuf::new();
    for p in path {
        if p.join(&req_exe_name).exists() {
            exe = p.join(&req_exe_name);
            break;
        }
    }
    assert!(
        exe.exists(),
        "Requested executable '{}' cannot be found anywhere in $PATH",
        req_exe_name,
    );

    exe
}

pub fn compose_environments(
    req_exe_name: String,
    config: BagexConfig,
) -> HashMap<String, String> {
    let mut ret: HashMap<String, String> = HashMap::new();

    log::debug!("Getting envs from env-exe mapping from config file ..");
    if let toml::Value::Table(env_table) = config.env.unwrap() {
        for (env_name, info) in env_table {
            if let toml::Value::Array(executables) =
                info.get("executables").unwrap()
            {
                if executables
                    .contains(&toml::Value::String(req_exe_name.clone()))
                {
                    let value = info.get("value").unwrap();
                    ret.insert(
                        env_name,
                        if value.is_str() {
                            value.as_str().unwrap().to_string()
                        } else {
                            value.to_string()
                        },
                    );
                }
            }
        }
    }

    log::debug!("Getting envs from exe-env mapping from config file ..");
    if let toml::Value::Table(exe_table) = config.exe.unwrap() {
        for (config_exe_name, env_specs) in exe_table {
            if config_exe_name == req_exe_name {
                if let toml::Value::Table(envs) = env_specs {
                    for (env_name, value) in envs {
                        ret.insert(
                            env_name,
                            if value.is_str() {
                                value.as_str().unwrap().to_string()
                            } else {
                                value.to_string()
                            },
                        );
                    }
                }
            }
        }
    }

    ret
}

// Author: Blurgy <gy@blurgy.xyz>
// Date:   Jul 26 2021, 11:07 [CST]
