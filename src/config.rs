use std::fs;

use serde::Deserialize;
use toml;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub email: Option<Email>,
}

#[derive(Deserialize, Debug)]
pub struct Email {
    pub server: String,
    pub user: String,
    pub password: String,
    pub from: String,
}

pub fn read_config() -> Config {
    // TODO: Use path relative to binary, not to CWD
    match fs::read_to_string("config.toml") {
        Ok(contents) => {
            toml::from_str(&contents).expect("Configuration invalid")
        },
        _ => Config {
            email: None
        }
    }
}
