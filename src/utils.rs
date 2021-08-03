use std::{
    collections::HashMap, os::unix::prelude::PermissionsExt, path::PathBuf,
    str::FromStr,
};

use color_eyre::Report;

use crate::config::BagexConfig;

fn get_home_dir() -> PathBuf {
    PathBuf::from_str(&std::env::var("HOME").unwrap_or("/".to_string()))
        .unwrap_or_default()
}

/// If the environment variable $XDXDG_CONFIG_HOME is set, returns
/// $XDG_CONFIG_HOME/bagex/config.toml, else returns $HOME/.config/bagex/config.toml
pub fn default_config_file() -> PathBuf {
    if let Ok(path) = std::env::var("XDG_CONFIG_HOME") {
        PathBuf::from_str(&path).unwrap()
    } else {
        get_home_dir().join(".config")
    }
    .join("bagex")
    .join("config.toml")
}

pub fn compose_and_set_env_path(
    additional_path: Vec<PathBuf>,
) -> Vec<PathBuf> {
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

pub fn get_exe_abs_path(
    req_exe_name: String,
    path: Vec<PathBuf>,
) -> Result<PathBuf, Report> {
    for p in path {
        let candidate = p.join(&req_exe_name);
        if candidate.is_file()
            && candidate.metadata()?.permissions().mode() & 0o110 == 0o110
        {
            return Ok(candidate);
        }
    }
    Err(color_eyre::eyre::eyre!(
        "Requested executable '{}' cannot be found anywhere in $PATH",
        req_exe_name,
    ))
}

pub fn compose_environments(
    req_exe_name: String,
    config: &BagexConfig,
) -> HashMap<String, String> {
    let mut ret: HashMap<String, String> = HashMap::new();

    log::debug!("Getting envs from env-exe mapping from config file ..");
    if let toml::Value::Table(env_table) = config.env.as_ref().unwrap() {
        for (env_name, info) in env_table {
            if let toml::Value::Table(ref value_exe_pairs) = info {
                for (value, exes) in value_exe_pairs {
                    if let toml::Value::Array(executables) = exes {
                        if executables.contains(&toml::Value::String(
                            req_exe_name.clone(),
                        )) {
                            assert!(
                        !ret.contains_key(env_name),
                        "Environment variable {} is specified multiple times",
                        env_name,
                    );
                            log::debug!(
                                "Setting env: \"{}={}\"",
                                env_name,
                                value
                            );
                            ret.insert(
                                env_name.to_string(),
                                value.to_string(),
                            );
                            break;
                        }
                    }
                }
            }
        }
    }

    log::debug!("Getting envs from exe-env mapping from config file ..");
    if let toml::Value::Table(exe_table) = config.exe.as_ref().unwrap() {
        for (config_exe_name, env_specs) in exe_table {
            if config_exe_name == &req_exe_name {
                if let toml::Value::Table(envs) = env_specs {
                    for (env_name, value_raw) in envs {
                        assert!(
                            !ret.contains_key(env_name),
                            "Environment variable {} is specified multiple times",
                            env_name,
                        );
                        let value = if value_raw.is_str() {
                            value_raw.as_str().unwrap().to_string()
                        } else {
                            value_raw.to_string()
                        };
                        log::debug!("Setting env: {}={}", env_name, value);
                        ret.insert(env_name.to_string(), value);
                    }
                }
            }
        }
    }

    ret
}

// Author: Blurgy <gy@blurgy.xyz>
// Date:   Jul 26 2021, 11:07 [CST]
