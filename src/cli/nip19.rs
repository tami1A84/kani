use clap::{Parser, Subcommand};
use nostr_sdk::prelude::*;

#[derive(Parser)]
pub struct Nip19Command {
    #[command(subcommand)]
    subcommand: Nip19Subcommand,
}

#[derive(Subcommand)]
pub enum Nip19Subcommand {
    /// Encode entities to bech32 format
    Encode(EncodeCommand),
    /// Decode a bech32 string
    Decode {
        /// Bech32 string to decode
        bech32_string: String,
    },
}

#[derive(Parser)]
pub struct EncodeCommand {
    #[command(subcommand)]
    subcommand: EncodeSubcommand,
}

#[derive(Subcommand)]
pub enum EncodeSubcommand {
    /// Encode a public key to npub format
    Npub {
        /// Public key in hex format
        hex_pubkey: String,
    },
    /// Encode a secret key to nsec format
    Nsec {
        /// Secret key in hex format
        hex_seckey: String,
    },
    /// Encode an event ID to note format
    Note {
        /// Event ID in hex format
        hex_event_id: String,
    },
    /// Encode a profile to nprofile format
    Nprofile {
        /// Public key in hex format
        hex_pubkey: String,
        /// Relays
        relays: Vec<String>,
    },
    /// Encode an event to nevent format
    Nevent {
        /// Event ID in hex format
        hex_event_id: String,
        /// Author public key in hex format
        #[clap(long)]
        author_pubkey: Option<String>,
        /// Relays
        relays: Vec<String>,
    },
}

pub async fn handle_nip19_command(command: Nip19Command) -> Result<(), Box<dyn std::error::Error>> {
    match command.subcommand {
        Nip19Subcommand::Decode { bech32_string } => {
            let decoded = Nip19::from_bech32(&bech32_string)?;
            println!("{:#?}", decoded);
        }
        Nip19Subcommand::Encode(encode_command) => match encode_command.subcommand {
            EncodeSubcommand::Npub { hex_pubkey } => {
                let pubkey = PublicKey::from_hex(&hex_pubkey)?;
                println!("{}", pubkey.to_bech32()?);
            }
            EncodeSubcommand::Nsec { hex_seckey } => {
                let seckey = SecretKey::from_hex(&hex_seckey)?;
                println!("{}", seckey.to_bech32()?);
            }
            EncodeSubcommand::Note { hex_event_id } => {
                let event_id = EventId::from_hex(&hex_event_id)?;
                println!("{}", event_id.to_bech32()?);
            }
            EncodeSubcommand::Nprofile { hex_pubkey, relays } => {
                let pubkey = PublicKey::from_hex(&hex_pubkey)?;
                let relays_url = relays.into_iter().map(|r| RelayUrl::parse(&r)).collect::<Result<Vec<_>, _>>()?;
                let profile = Nip19Profile::new(pubkey, relays_url);
                println!("{}", profile.to_bech32()?);
            }
            EncodeSubcommand::Nevent { hex_event_id, author_pubkey, relays } => {
                let event_id = EventId::from_hex(&hex_event_id)?;
                let author = match author_pubkey {
                    Some(hex) => Some(PublicKey::from_hex(&hex)?),
                    None => None,
                };
                let relays_url = relays.into_iter().map(|r| RelayUrl::parse(&r)).collect::<Result<Vec<_>, _>>()?;
                let mut event = Nip19Event::new(event_id);
                event.author = author;
                event.relays = relays_url;
                println!("{}", event.to_bech32()?);
            }
        },
    }
    Ok(())
}
