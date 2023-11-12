use serde::{Deserialize, Serialize};
use std::fs;
use toml;

const CONFIG_PATH: &str = "data/config.toml";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub chat_id: String,
    pub bot_token: String,
    pub item: Vec<Search>,
}
#[derive(Serialize, Deserialize)]
pub struct Search {
    pub keyword: String,
    #[serde(default)]
    pub region: Option<Vec<u8>>,
    #[serde(default)]
    pub province: Option<u8>,
    #[serde(default)]
    pub city: Option<String>,
    #[serde(default)]
    pub search_only_title: Option<bool>,
    #[serde(default)]
    pub category: Option<u8>,
    #[serde(default)]
    pub min_price: Option<i32>,
    #[serde(default)]
    pub max_price: Option<i32>,
    #[serde(default)]
    pub last_listing: Option<String>,
}

pub fn get_config() -> Config {
    let config = fs::read_to_string(CONFIG_PATH).unwrap();
    let config: Config = toml::from_str(config.as_str()).unwrap();
    config
}

pub fn set_config(config: &Config) {
    let config = toml::to_string(config).unwrap();
    fs::write(CONFIG_PATH, config).unwrap();
}