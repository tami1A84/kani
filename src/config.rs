use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Config {
    pub secret_key: Option<String>,
    pub relays: Option<Vec<String>>,
    pub encrypted_secret_key: Option<String>,
}

use crate::error::Error;

pub fn get_config_path() -> Result<PathBuf, Error> {
    let path = dirs::config_dir()
        .ok_or(Error::Message(
            "Could not find config directory".to_string(),
        ))?
        .join("kani");
    fs::create_dir_all(&path)?;
    Ok(path.join("config.toml"))
}

pub fn load_config() -> Result<Config, Error> {
    let config_path = get_config_path()?;

    if !config_path.exists() {
        return Ok(Config::default());
    }

    let content = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&content)?;

    Ok(config)
}

pub fn save_config(config: &Config) -> Result<(), Error> {
    let config_path = get_config_path()?;
    let content = toml::to_string(config)?;
    fs::write(config_path, content)?;
    Ok(())
}
