use std::{path::PathBuf, str::FromStr};

fn get_home_dir() -> PathBuf {
    PathBuf::from_str(&std::env::var("HOME").unwrap_or("/".to_string()))
        .unwrap_or_default()
}

/// If the environment variable $XDXDG_CONFIG_HOME is set, returns
/// $XDG_CONFIG_HOME/bages/config.toml, else returns $HOME/.config/bagex/config.toml
pub fn default_config_path() -> PathBuf {
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

// Author: Blurgy <gy@blurgy.xyz>
// Date:   Jul 26 2021, 11:07 [CST]
