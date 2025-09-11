use clap::{Parser, Subcommand};
use nostr_sdk::prelude::*;
use nostr::prelude::{ToBech32, FromBech32};
use nostr::bip39::Mnemonic;
use nostr::nips::nip49::EncryptedSecretKey;
use nostr::{Keys, SecretKey};

#[derive(Parser, Clone)]
pub struct KeyCommand {
    #[command(subcommand)]
    subcommand: KeySubcommand,
}

#[derive(Subcommand, Clone)]
enum KeySubcommand {
    /// Generate new keys
    Generate,
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
        KeySubcommand::Encrypt { secret_key, password } => {
            let sk = SecretKey::from_bech32(&secret_key)?;
            let encrypted = sk.encrypt(&password)?;
            println!("{}", encrypted.to_bech32()?);
        }
        KeySubcommand::Decrypt { encrypted_key, password } => {
            let encrypted = EncryptedSecretKey::from_bech32(&encrypted_key)?;
            let sk = encrypted.decrypt(&password)?;
            println!("{}", sk.to_bech32()?);
        }
    }
    Ok(())
}
