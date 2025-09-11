use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Default, Debug)]
pub struct Config {
    pub secret_key: Option<String>,
    pub relays: Option<Vec<String>>,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = dirs::config_dir()
        .ok_or("Could not find config directory")?
        .join("kani/config.toml");

    if !config_path.exists() {
        return Ok(Config::default());
    }

    let content = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&content)?;

    Ok(config)
}
