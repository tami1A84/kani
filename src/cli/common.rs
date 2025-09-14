use crate::cli::CommonOptions;
use crate::config::Config;
use crate::error::Error;
use nostr_sdk::{Client, Keys};

pub fn get_secret_key(
    common_opts: &CommonOptions,
    config: &Config,
) -> Result<String, Error> {
    if let Some(sk) = &common_opts.secret_key {
        return Ok(sk.clone());
    }
    if let Ok(sk) = std::env::var("NOSTR_SECRET_KEY") {
        return Ok(sk);
    }
    if let Some(sk) = &config.secret_key {
        return Ok(sk.clone());
    }
    Err(Error::SecretKeyMissing)
}

pub fn get_relays(common_opts: &CommonOptions, config: &Config) -> Vec<String> {
    if !common_opts.relay.is_empty() {
        common_opts.relay.clone()
    } else {
        config.relays.clone().unwrap_or_default()
    }
}

pub async fn connect_client(keys: Keys, relays: Vec<String>) -> Result<Client, Error> {
    if relays.is_empty() {
        return Err(Error::Message(
            "No relays provided in args or config".to_string(),
        ));
    }

    let client = Client::new(keys);
    for relay in relays {
        client.add_relay(relay).await?;
    }
    client.connect().await;

    Ok(client)
}
