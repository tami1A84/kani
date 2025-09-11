use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Default, Debug)]
pub struct Config {
    pub secret_key: Option<String>,
    pub relays: Option<Vec<String>>,
}

pub fn get_config_path() -> Result<PathBuf, &'static str> {
    dirs::config_dir()
        .ok_or("Could not find config directory")
        .map(|p| p.join("kani/config.toml"))
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = get_config_path()?;

    if !config_path.exists() {
        return Ok(Config::default());
    }

    let content = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&content)?;

    Ok(config)
}
