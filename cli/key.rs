use clap::{Parser, Subcommand};
use nostr_sdk::prelude::*;
use nostr::prelude::{ToBech32};
use nostr::bip39::Mnemonic;
use nostr::Keys;

#[derive(Parser)]
pub struct KeyCommand {
    #[command(subcommand)]
    subcommand: KeySubcommand,
}

#[derive(Subcommand)]
enum KeySubcommand {
    /// Generate new keys
    Generate,
    /// Derives keys from a mnemonic
    FromMnemonic {
        /// Mnemonic phrase
        mnemonic: String,
    },
}

pub async fn handle_key_command(command: KeyCommand) -> Result<(), Box<dyn std::error::Error>> {
    match command.subcommand {
        KeySubcommand::Generate => {
            let keys = Keys::generate();
            println!("Public key: {}", keys.public_key().to_bech32()?);
            println!("Secret key: {}", keys.secret_key().to_bech32()?);
        }
        KeySubcommand::FromMnemonic { mnemonic } => {
            let mnemonic = Mnemonic::parse(&mnemonic)?;
            let keys = Keys::from_mnemonic(mnemonic.to_string(), None)?;
            println!("Public key: {}", keys.public_key().to_bech32()?);
            println!("Secret key: {}", keys.secret_key().to_bech32()?);
        }
    }
    Ok(())
}
