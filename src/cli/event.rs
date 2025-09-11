use crate::cli::CommonOptions;
use crate::config::load_config;
use clap::{Parser, Subcommand};
use nostr::nips::{nip04, nip44};
use nostr::prelude::{FromBech32, ToBech32};
use nostr::{EventBuilder, Keys, SecretKey};
use nostr_sdk::nips::nip09::EventDeletionRequest;
use nostr_sdk::prelude::*;
use std::time::Duration;

#[derive(Parser, Clone)]
pub struct EventCommand {
    #[command(subcommand)]
    subcommand: EventSubcommand,
    #[command(flatten)]
    common: CommonOptions,
}

#[derive(Subcommand, Clone)]
enum EventSubcommand {
    /// Create a text note
    CreateTextNote {
        /// Text note content
        content: String,
        /// Recipient public key for gift wrap (NIP-59)
        #[clap(long)]
        gift_wrap_recipient: Option<String>,
    },
    /// Create a direct message (NIP-04)
    CreateDm {
        /// Recipient public key (bech32)
        #[clap(short, long)]
        recipient: String,
        /// DM content
        content: String,
    },
    /// Get an event by id
    Get {
        /// Event id to get
        id: String,
    },
    /// Delete an event
    Delete {
        /// ID of the event to delete
        event_id: String,
    },
    /// Encrypt a payload using NIP-44
    EncryptPayload {
        /// Recipient public key (bech32)
        #[clap(short, long)]
        recipient: String,
        /// Content to encrypt
        content: String,
    },
    /// Decrypt a payload using NIP-44
    DecryptPayload {
        /// Sender public key (bech32)
        #[clap(short, long)]
        sender: String,
        /// Encrypted content
        content: String,
    },
    /// Create a long-form content note (NIP-23)
    CreateLongFormPost {
        /// Path to the markdown file
        #[clap(short, long)]
        file: String,
        /// Title of the article
        #[clap(long)]
        title: Option<String>,
        /// Summary of the article
        #[clap(long)]
        summary: Option<String>,
        /// `d` identifier for the article
        #[clap(long)]
        d_identifier: Option<String>,
    },
}

pub async fn handle_event_command(command: EventCommand) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;

    let relays = if !command.common.relay.is_empty() {
        command.common.relay
    } else {
        config.relays.clone().unwrap_or_default()
    };

    match command.subcommand {
        EventSubcommand::CreateTextNote {
            content,
            gift_wrap_recipient,
        } => {
            if relays.is_empty() {
                return Err("No relays provided in args or config".into());
            }
            let secret_key_str = command.common.secret_key.or(config.secret_key).ok_or("Secret key not provided")?;
            let secret_key = SecretKey::from_bech32(&secret_key_str)?;
            let keys = Keys::new(secret_key);
            let client = Client::new(keys.clone());
            for relay in relays {
                client.add_relay(&relay).await?;
            }
            client.connect().await;

            let builder = EventBuilder::text_note(&content);

            let event_to_send = if let Some(recipient_str) = gift_wrap_recipient {
                let recipient_pk = PublicKey::from_bech32(&recipient_str)?;
                let rumor = builder.build(keys.public_key());
                EventBuilder::gift_wrap(&keys, &recipient_pk, rumor, []).await?
            } else {
                client.sign_event_builder(builder).await?
            };

            let event_id = client.send_event(&event_to_send).await?;
            println!("Event sent with id: {}", event_id.to_bech32()?);

            client.shutdown().await;
        }
        EventSubcommand::CreateDm {
            recipient,
            content,
        } => {
            if relays.is_empty() {
                return Err("No relays provided in args or config".into());
            }
            let secret_key_str = command.common.secret_key.or(config.secret_key).ok_or("Secret key not provided")?;
            let secret_key = SecretKey::from_bech32(&secret_key_str)?;
            let keys = Keys::new(secret_key);
            let recipient_pubkey = PublicKey::from_bech32(&recipient)?;

            let client = Client::new(keys.clone());
            for relay in relays {
                client.add_relay(&relay).await?;
            }
            client.connect().await;

            let sk = keys.secret_key();
            let encrypted_content = nip04::encrypt(sk, &recipient_pubkey, &content)?;
            let builder = EventBuilder::new(Kind::EncryptedDirectMessage, encrypted_content)
                .tag(Tag::public_key(recipient_pubkey));
            let event = client.sign_event_builder(builder).await?;
            let event_id = client.send_event(&event).await?;
            println!("DM event sent with id: {}", event_id.to_bech32()?);

            client.shutdown().await;
        }
        EventSubcommand::Get { id } => {
            if relays.is_empty() {
                return Err("No relays provided in args or config".into());
            }
            let event_id =
                EventId::from_bech32(&id).or_else(|_| EventId::from_hex(&id))?;

            let keys = Keys::generate();
            let client = Client::new(keys);

            let relay_urls: Vec<&str> = relays.iter().map(|s| s.as_str()).collect();

            let filter = Filter::new().id(event_id);
            let timeout = Duration::from_secs(10);
            let events = client
                .fetch_events_from(relay_urls, filter, timeout)
                .await?;

            if let Some(event) = events.first() {
                println!("{:#?}", event);
            } else {
                println!("Event not found.");
            }

            client.shutdown().await;
        }
        EventSubcommand::Delete { event_id } => {
            if relays.is_empty() {
                return Err("No relays provided in args or config".into());
            }
            let secret_key_str = command.common.secret_key.or(config.secret_key).ok_or("Secret key not provided")?;
            let secret_key = SecretKey::from_bech32(&secret_key_str)?;
            let keys = Keys::new(secret_key);
            let client = Client::new(keys);
            for relay in relays {
                client.add_relay(&relay).await?;
            }
            client.connect().await;

            let event_id_to_delete =
                EventId::from_bech32(&event_id).or_else(|_| EventId::from_hex(&event_id))?;

            let request = EventDeletionRequest {
                ids: vec![event_id_to_delete],
                coordinates: vec![],
                reason: None,
            };
            let builder = EventBuilder::delete(request);
            let signed_event = client.sign_event_builder(builder).await?;
            let deletion_event_id = client.send_event(&signed_event).await?;
            println!(
                "Deletion event sent with id: {}",
                deletion_event_id.to_bech32()?
            );

            client.shutdown().await;
        }
        EventSubcommand::EncryptPayload {
            recipient,
            content,
        } => {
            let secret_key_str = command.common.secret_key.or(config.secret_key).ok_or("Secret key not provided")?;
            let sk = SecretKey::from_bech32(&secret_key_str)?;
            let pk = PublicKey::from_bech32(&recipient)?;
            let encrypted = nip44::encrypt(&sk, &pk, &content, nip44::Version::default())?;
            println!("{}", encrypted);
        }
        EventSubcommand::DecryptPayload { sender, content } => {
            let secret_key_str = command.common.secret_key.or(config.secret_key).ok_or("Secret key not provided")?;
            let sk = SecretKey::from_bech32(&secret_key_str)?;
            let pk = PublicKey::from_bech32(&sender)?;
            let decrypted = nip44::decrypt(&sk, &pk, &content)?;
            println!("{}", decrypted);
        }
        EventSubcommand::CreateLongFormPost {
            file,
            title,
            summary,
            d_identifier,
        } => {
            if relays.is_empty() {
                return Err("No relays provided in args or config".into());
            }
            let secret_key_str = command.common.secret_key.or(config.secret_key).ok_or("Secret key not provided")?;
            let secret_key = SecretKey::from_bech32(&secret_key_str)?;
            let keys = Keys::new(secret_key);
            let client = Client::new(keys.clone());
            for relay in relays {
                client.add_relay(&relay).await?;
            }
            client.connect().await;

            let content = std::fs::read_to_string(&file)?;

            let d_tag_value = d_identifier.unwrap_or_else(|| {
                std::path::Path::new(&file)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("default-d-identifier")
                    .to_string()
            });

            let mut tags: Vec<Tag> = vec![Tag::identifier(d_tag_value)];

            if let Some(title) = title {
                tags.push(Tag::parse(["title", &title.as_str()])?);
            }
            if let Some(summary) = summary {
                tags.push(Tag::parse(["summary", &summary.as_str()])?);
            }

            let publication_timestamp = Timestamp::now();
            let timestamp_str = publication_timestamp.as_u64().to_string();
            tags.push(Tag::parse(["published_at", &timestamp_str])?);

            let builder = EventBuilder::new(Kind::Custom(30023), &content).tags(tags);
            let event = client.sign_event_builder(builder).await?;
            let event_id = client.send_event(&event).await?;
            println!(
                "Long-form post sent with id: {}",
                event_id.to_bech32()?
            );

            client.shutdown().await;
        }
    }
    Ok(())
}
