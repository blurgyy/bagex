use serde::Deserialize;

#[derive(Default, Clone, Deserialize, Debug)]
pub struct BagexConfig {
    pub path: Option<Vec<std::path::PathBuf>>,
    pub env: Option<toml::Value>,
    pub exe: Option<toml::Value>,
}

// Author: Blurgy <gy@blurgy.xyz>
// Date:   Jul 25 2021, 23:50 [CST]
