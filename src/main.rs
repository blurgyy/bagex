use std::fs;

use bagex::config;

fn main() {
    let confstr = fs::read_to_string("./config.toml").unwrap_or_default();
    let config: config::BagexConfig = toml::from_str(&confstr).unwrap();
    println!("{:#?}", config);
}
