use crate::cli::CommonOptions;
use crate::config::load_config;
use clap::{Parser, Subcommand};
use nostr::prelude::FromBech32;
use nostr::{Keys, SecretKey};
use nostr_sdk::prelude::*;
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
    }
    Ok(())
}
