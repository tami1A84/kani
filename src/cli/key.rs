use crate::cli::{event, relay};
use crate::config::{load_config, save_config};
use clap::{Parser, Subcommand};
use dialoguer::{Confirm, Password, theme::ColorfulTheme};
use nostr::bip39::Mnemonic;
use nostr::nips::nip49::EncryptedSecretKey;
use nostr::prelude::{FromBech32, ToBech32};
use nostr::{Keys, SecretKey};
use nostr_sdk::prelude::*;
use crate::cli::CommonOptions;

#[derive(Parser, Clone)]
pub struct KeyCommand {
    #[command(subcommand)]
    subcommand: KeySubcommand,
    #[command(flatten)]
    common: CommonOptions,
}

#[derive(Subcommand, Clone)]
enum KeySubcommand {
    /// Generate new keys
    Generate {
        /// Start the onboarding wizard after generating keys
        #[clap(long, default_value_t = false)]
        wizard: bool,
    },
    /// Derives keys from a mnemonic
    FromMnemonic {
        /// Mnemonic phrase
        mnemonic: String,
    },
    /// Encrypt a secret key (NIP-49)
    Encrypt {
        /// Secret key to encrypt (bech32)
        #[clap(short, long)]
        secret_key: String,
        /// Password
        #[clap(short, long)]
        password: String,
    },
    /// Decrypt a secret key (NIP-49)
    Decrypt {
        /// Encrypted secret key (bech32)
        #[clap(short, long)]
        encrypted_key: String,
        /// Password
        #[clap(short, long)]
        password: String,
    },
}

use crate::error::Error;

use crate::cli::common::get_relays;

pub async fn handle_key_command(command: KeyCommand) -> Result<(), Error> {
    match command.subcommand {
        KeySubcommand::Generate { wizard } => {
            let keys = Keys::generate();
            let secret_key_bech32 = keys.secret_key().to_bech32().unwrap();
            println!("Public key: {}", keys.public_key().to_bech32().unwrap());
            println!("Secret key: {}", secret_key_bech32);

            if wizard {
                println!("\nStarting onboarding wizard...");
                let theme = ColorfulTheme::default();
                let config = load_config()?;
                let relays = get_relays(&command.common, &config);

                if Confirm::with_theme(&theme)
                    .with_prompt("Do you want to set up your profile now?")
                    .default(true)
                    .interact()?
                {
                    event::edit_profile(secret_key_bech32.clone(), relays.clone()).await?;
                }

                if Confirm::with_theme(&theme)
                    .with_prompt("Do you want to set up your relay list now?")
                    .default(true)
                    .interact()?
                {
                    relay::edit_relays(secret_key_bech32.clone(), relays).await?;
                }

                if Confirm::with_theme(&theme)
                    .with_prompt("Do you want to save your secret key to the config file (encrypted)?")
                    .default(true)
                    .interact()?
                {
                    let password = Password::with_theme(&theme)
                        .with_prompt("Enter a password to encrypt your secret key")
                        .interact()?;
                    let sk = SecretKey::from_bech32(&secret_key_bech32)?;
                    let encrypted_key = sk.encrypt(&password)?;
                    let mut config = load_config()?;
                    config.encrypted_secret_key = Some(encrypted_key.to_bech32()?);
                    save_config(&config)?;
                    println!("Encrypted secret key saved to config file.");
                }

                println!("\nOnboarding complete!");
            }
        }
        KeySubcommand::FromMnemonic { mnemonic } => {
            let mnemonic = Mnemonic::parse(&mnemonic)?;
            let keys = Keys::from_mnemonic(mnemonic.to_string(), None)?;
            println!("Public key: {}", keys.public_key().to_bech32().unwrap());
            println!("Secret key: {}", keys.secret_key().to_bech32().unwrap());
        }
        KeySubcommand::Encrypt {
            secret_key,
            password,
        } => {
            let sk = SecretKey::from_bech32(&secret_key)?;
            let encrypted = sk.encrypt(&password)?;
            println!("{}", encrypted.to_bech32().unwrap());
        }
        KeySubcommand::Decrypt {
            encrypted_key,
            password,
        } => {
            let encrypted = EncryptedSecretKey::from_bech32(&encrypted_key)?;
            let sk = encrypted.decrypt(&password)?;
            println!("{}", sk.to_bech32().unwrap());
        }
    }
    Ok(())
}
