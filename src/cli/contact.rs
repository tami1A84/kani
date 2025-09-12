use crate::cli::CommonOptions;
use crate::cli::common::{connect_client, get_relays};
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
    /// Add a contact to your list
    Add {
        /// Public keys to follow
        pubkeys: Vec<String>,
    },
    /// List contacts
    List {
        /// Public key to get the contact list for
        pubkey: String,
    },
}

use crate::error::Error;

pub async fn handle_contact_command(command: ContactCommand) -> Result<(), Error> {
    let config = load_config()?;
    let relays = get_relays(&command.common, &config);

    match command.subcommand {
        ContactSubcommand::Add { pubkeys } => {
            let secret_key_str = command
                .common
                .secret_key
                .or(config.secret_key.clone())
                .ok_or(Error::SecretKeyMissing)?;
            set_contact_list(pubkeys, secret_key_str, relays).await?;
        }
        ContactSubcommand::List { pubkey } => {
            get_contact_list(pubkey, relays).await?;
        }
    }
    Ok(())
}

async fn set_contact_list(
    pubkeys: Vec<String>,
    secret_key_str: String,
    relays: Vec<String>,
) -> Result<(), Error> {
    let secret_key = SecretKey::from_bech32(&secret_key_str)?;
    let keys = Keys::new(secret_key);
    let client = connect_client(keys, relays).await?;

    let mut contacts = Vec::new();
    for pubkey_str in pubkeys {
        let pubkey = if let Ok(pk) = PublicKey::from_bech32(&pubkey_str) {
            pk
        } else {
            PublicKey::from_hex(&pubkey_str)?
        };
        contacts.push(Contact::new(pubkey));
    }

    let builder = EventBuilder::contact_list(contacts);
    let event = client.sign_event_builder(builder).await?;
    client.send_event(&event).await?;
    println!("Contact list updated.");

    client.shutdown().await;
    Ok(())
}

async fn get_contact_list(pubkey: String, relays: Vec<String>) -> Result<(), Error> {
    if relays.is_empty() {
        return Err(Error::Message(
            "No relays provided in args or config".to_string(),
        ));
    }
    let pubkey = if let Ok(pk) = PublicKey::from_bech32(&pubkey) {
        pk
    } else {
        PublicKey::from_hex(&pubkey)?
    };

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
    Ok(())
}
