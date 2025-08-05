use clap::{Parser, Subcommand};
use nostr_sdk::prelude::*;
use nostr_sdk::RelayUrl;
use std::str::FromStr;

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
        /// Kind
        #[clap(long)]
        kind: Option<u16>,
        /// Relays
        relays: Vec<String>,
    },
    // Naddr removed due to library limitations
}

pub async fn handle_nip19_command(command: Nip19Command) -> Result<(), Box<dyn std::error::Error>> {
    match command.subcommand {
        Nip19Subcommand::Decode { bech32_string } => {
            let decoded = Nip19::from_bech32(&bech32_string)?;
            match decoded {
                Nip19::Secret(seckey) => {
                    println!("nsec: {}", seckey.to_secret_hex());
                }
                Nip19::Pubkey(pubkey) => {
                    println!("npub: {}", pubkey.to_string());
                }
                Nip19::EventId(event_id) => {
                    println!("note: {}", event_id.to_hex());
                }
                Nip19::Profile(profile) => {
                    println!("nprofile:");
                    println!("  public_key: {}", profile.public_key.to_string());
                    println!("  relays: {:?}", profile.relays);
                }
                Nip19::Event(event_pointer) => {
                    println!("nevent:");
                    println!("  event_id: {}", event_pointer.event_id.to_hex());
                    if let Some(author) = event_pointer.author {
                        println!("  author: {}", author.to_string());
                    }
                    if let Some(kind) = event_pointer.kind {
                        println!("  kind: {}", kind);
                    }
                    println!("  relays: {:?}", event_pointer.relays);
                }
                Nip19::Coordinate(_coordinate) => {
                    println!("naddr decoding not fully supported in this version.");
                }
            }
        }
        Nip19Subcommand::Encode(encode_command) => {
            let encoded = match encode_command.subcommand {
                EncodeSubcommand::Npub { hex_pubkey } => {
                    let pubkey = PublicKey::from_str(&hex_pubkey)?;
                    pubkey.to_bech32()?
                }
                EncodeSubcommand::Nsec { hex_seckey } => {
                    let seckey = SecretKey::from_str(&hex_seckey)?;
                    seckey.to_bech32()?
                }
                EncodeSubcommand::Note { hex_event_id } => {
                    let event_id = EventId::from_hex(&hex_event_id)?;
                    event_id.to_bech32()?
                }
                EncodeSubcommand::Nprofile { hex_pubkey, relays } => {
                    let pubkey = PublicKey::from_str(&hex_pubkey)?;
                    let relays = relays
                        .iter()
                        .map(|s| RelayUrl::parse(s))
                        .collect::<Result<Vec<_>, _>>()?;
                    let profile = Nip19Profile::new(pubkey, relays);
                    profile.to_bech32()?
                }
                EncodeSubcommand::Nevent {
                    hex_event_id,
                    author_pubkey,
                    kind,
                    relays,
                } => {
                    let event_id = EventId::from_hex(&hex_event_id)?;
                    let author = author_pubkey
                        .map(|p| PublicKey::from_str(&p))
                        .transpose()?;
                    let kind = kind.map(|k| k.into());
                    let relays = relays
                        .iter()
                        .map(|s| RelayUrl::parse(s))
                        .collect::<Result<Vec<_>, _>>()?;

                    let mut nevent = Nip19Event::new(event_id);
                    nevent = nevent.relays(relays);
                    if let Some(author) = author {
                        nevent = nevent.author(author);
                    }
                    if let Some(kind) = kind {
                        nevent = nevent.kind(kind);
                    }
                    nevent.to_bech32()?
                }
            };
            println!("{}", encoded);
        }
    }
    Ok(())
}
