use std::path::PathBuf;

use color_eyre::Report;
use serde::Deserialize;

#[derive(Default, Clone, Deserialize, Debug)]
pub struct BagexConfig {
    pub path: Option<Vec<std::path::PathBuf>>,
    pub env: Option<toml::Value>,
    pub exe: Option<toml::Value>,
}

impl BagexConfig {
    pub fn from_pathbuf(path: PathBuf) -> Result<BagexConfig, Report> {
        let confstr = std::fs::read_to_string(path).unwrap();
        Ok(toml::from_str(&confstr).unwrap())
    }
}

// Author: Blurgy <gy@blurgy.xyz>
// Date:   Jul 25 2021, 23:50 [CST]
