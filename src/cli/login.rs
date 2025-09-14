use crate::config::load_config;
use crate::error::Error;
use clap::Parser;
use dialoguer::Password;
use nostr::nips::nip49::EncryptedSecretKey;
use nostr::prelude::{FromBech32, ToBech32};

#[derive(Parser, Clone)]
pub struct LoginCommand {}

pub async fn handle_login_command(_command: LoginCommand) -> Result<(), Error> {
    let config = load_config()?;

    let encrypted_key_bech32 = config.encrypted_secret_key.ok_or(Error::Message(
        "No encrypted secret key found in config. Please run `key generate --wizard` or `key encrypt` first.".to_string(),
    ))?;

    let password = Password::new()
        .with_prompt("Enter password to decrypt secret key")
        .interact()?;

    let encrypted_key = EncryptedSecretKey::from_bech32(&encrypted_key_bech32)?;
    let secret_key = encrypted_key.decrypt(&password)?;

    println!(
        "export NOSTR_SECRET_KEY={}",
        secret_key.to_bech32().unwrap()
    );
    // stderr message to the user so it doesn't get captured by eval
    eprintln!("Login successful. Key is now available in your shell environment.");
    eprintln!("Run `eval $(kani-nostr-cli logout)` to clear the key.");

    Ok(())
}
