use crate::cli::CommonOptions;
use crate::config::load_config;
use clap::{Parser, Subcommand};
use nostr::prelude::FromBech32;
use nostr::{Keys, SecretKey};
use nostr_sdk::prelude::*;
use std::env;
use std::fs;
use std::io::Write;
use std::process::Command as StdCommand;
use std::time::Duration;

#[derive(Parser, Clone)]
pub struct ContactCommand {
    #[command(subcommand)]
    subcommand: ContactSubcommand,
    #[command(flatten)]
    common: CommonOptions,
}

#[derive(Subcommand, Clone)]
pub enum ContactSubcommand {
    /// Set contact list
    Set {
        /// Public keys to follow
        pubkeys: Vec<String>,
    },
    /// Get contact list
    Get {
        /// Public key to get the contact list for
        pubkey: String,
    },
    /// Set relay list (NIP-65)
    SetRelays {
        /// Relays to include in the list. Format: wss://relay.example.com[#read|#write]
        relays: Vec<String>,
    },
    /// Get relay list (NIP-65)
    GetRelays {
        /// Public key
        #[clap(short, long)]
        pubkey: String,
    },
    /// Edit relay list (NIP-65) in your editor
    EditRelays,
}

pub async fn handle_contact_command(
    command: ContactCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;

    let relays = if !command.common.relay.is_empty() {
        command.common.relay.clone()
    } else {
        config.relays.clone().unwrap_or_default()
    };

    match command.subcommand {
        ContactSubcommand::Set { pubkeys } => {
            if relays.is_empty() {
                return Err("No relays provided in args or config".into());
            }
            let secret_key_str = command
                .common
                .secret_key
                .or(config.secret_key.clone())
                .ok_or("Secret key not provided in args or config")?;

            let secret_key = SecretKey::from_bech32(&secret_key_str)?;
            let keys = Keys::new(secret_key);
            let client = Client::new(keys);
            for relay in relays {
                client.add_relay(&relay).await?;
            }
            client.connect().await;

            let mut contacts = Vec::new();
            for pubkey_str in pubkeys {
                let pubkey = PublicKey::from_bech32(&pubkey_str)
                    .or_else(|_| PublicKey::from_hex(&pubkey_str))?;
                contacts.push(Contact::new(pubkey));
            }

            let builder = EventBuilder::contact_list(contacts);
            let event = client.sign_event_builder(builder).await?;
            client.send_event(&event).await?;
            println!("Contact list updated.");

            client.shutdown().await;
        }
        ContactSubcommand::Get { pubkey } => {
            if relays.is_empty() {
                return Err("No relays provided in args or config".into());
            }
            let pubkey =
                PublicKey::from_bech32(&pubkey).or_else(|_| PublicKey::from_hex(&pubkey))?;

            let keys = Keys::generate();
            let client = Client::new(keys);

            let relay_urls: Vec<&str> = relays.iter().map(|s| s.as_str()).collect();

            let filter = Filter::new()
                .author(pubkey)
                .kind(Kind::ContactList)
                .limit(1);

            let timeout = Duration::from_secs(10);
            let events = client
                .fetch_events_from(relay_urls, filter, timeout)
                .await?;

            if let Some(event) = events.first() {
                println!("{:#?}", event.tags);
            } else {
                println!("Contact list not found.");
            }

            client.shutdown().await;
        }
        ContactSubcommand::SetRelays { relays: relays_to_set } => {
            if relays.is_empty() {
                return Err("No relays provided in args or config to publish the list".into());
            }
            let secret_key_str = command
                .common
                .secret_key
                .or(config.secret_key)
                .ok_or("Secret key not provided in args or config")?;

            let secret_key = SecretKey::from_bech32(&secret_key_str)?;
            let keys = Keys::new(secret_key);
            let client = Client::new(keys);
            for relay in relays {
                client.add_relay(&relay).await?;
            }
            client.connect().await;

            let mut tags = Vec::new();
            for r in relays_to_set {
                let mut parts = r.splitn(2, '#');
                let url = parts.next().unwrap();
                let marker = parts.next();

                let tag_vec = if let Some(m) = marker {
                    if m == "read" || m == "write" {
                        vec!["r".to_string(), url.to_string(), m.to_string()]
                    } else {
                        vec!["r".to_string(), url.to_string()]
                    }
                } else {
                    vec!["r".to_string(), url.to_string()]
                };
                tags.push(Tag::parse(&tag_vec)?);
            }

            let builder = EventBuilder::new(Kind::RelayList, "").tags(tags);
            let event = client.sign_event_builder(builder).await?;
            client.send_event(&event).await?;
            println!("Relay list updated.");

            client.shutdown().await;
        }
        ContactSubcommand::GetRelays { pubkey } => {
             if relays.is_empty() {
                return Err("No relays provided in args or config".into());
            }
            let pubkey =
                PublicKey::from_bech32(&pubkey).or_else(|_| PublicKey::from_hex(&pubkey))?;

            let keys = Keys::generate();
            let client = Client::new(keys);

            let relay_urls: Vec<&str> = relays.iter().map(|s| s.as_str()).collect();

            let filter = Filter::new()
                .author(pubkey)
                .kind(Kind::RelayList)
                .limit(1);

            let timeout = Duration::from_secs(10);
            let events = client
                .fetch_events_from(relay_urls, filter, timeout)
                .await?;

            if let Some(event) = events.first() {
                println!("{:#?}", event.tags);
            } else {
                println!("Relay list not found.");
            }

            client.shutdown().await;
        }
        ContactSubcommand::EditRelays => {
            if relays.is_empty() {
                return Err("No relays provided in args or config".into());
            }
            // Get keys
            let secret_key_str = command
                .common
                .secret_key
                .or(config.secret_key)
                .ok_or("Secret key not provided in args or config")?;
            let secret_key = SecretKey::from_bech32(&secret_key_str)?;
            let keys = Keys::new(secret_key);
            let client = Client::new(keys.clone());

            // Add relays and connect
            for relay in &relays {
                client.add_relay(relay.clone()).await?;
            }
            client.connect().await;

            // Fetch existing relay list
            let filter = Filter::new()
                .author(keys.public_key())
                .kind(Kind::RelayList)
                .limit(1);
            let timeout = Duration::from_secs(10);
            let relay_urls: Vec<&str> = relays.iter().map(|s| s.as_str()).collect();
            let events = client
                .fetch_events_from(relay_urls, filter, timeout)
                .await?;

            let mut current_relays_str = String::new();
            if let Some(event) = events.first() {
                for tag in event.tags.iter() {
                    let tag_vec = tag.clone().to_vec();
                    if tag_vec.get(0).map(|s| s.as_str()) == Some("r") {
                        if let Some(url) = tag_vec.get(1) {
                            current_relays_str.push_str(url);
                            if let Some(marker) = tag_vec.get(2) {
                                if marker == "read" || marker == "write" {
                                    current_relays_str.push_str(&format!(" #{}", marker));
                                }
                            }
                            current_relays_str.push('\n');
                        }
                    }
                }
            }

            // Create and write to temp file
            let mut temp_file = tempfile::NamedTempFile::new()?;
            temp_file.write_all(current_relays_str.as_bytes())?;

            // Open editor
            let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
            let status = StdCommand::new(editor)
                .arg(temp_file.path())
                .status()?;

            if !status.success() {
                return Err("Editor exited with a non-zero status.".into());
            }

            // Read from temp file
            let new_relays_str = fs::read_to_string(temp_file.path())?;

            // Parse new relays
            let mut tags = Vec::new();
             for line in new_relays_str.lines().filter(|l| !l.trim().is_empty()) {
                let mut parts = line.splitn(2, '#');
                let url = parts.next().unwrap().trim();
                let marker = parts.next().map(|s| s.trim());

                let tag_vec = if let Some(m) = marker {
                    if m == "read" || m == "write" {
                        vec!["r".to_string(), url.to_string(), m.to_string()]
                    } else {
                        vec!["r".to_string(), url.to_string()]
                    }
                } else {
                    vec!["r".to_string(), url.to_string()]
                };
                tags.push(Tag::parse(&tag_vec)?);
            }

            // Publish new event using the same connected client
            let builder = EventBuilder::new(Kind::RelayList, "").tags(tags);
            let event = client.sign_event_builder(builder).await?;
            client.send_event(&event).await?;
            println!("Relay list updated.");

            client.shutdown().await;
        }
    }
    Ok(())
}
