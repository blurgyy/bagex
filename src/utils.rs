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

// Author: Blurgy <gy@blurgy.xyz>
// Date:   Jul 26 2021, 11:07 [CST]
