use clap::{Parser, Subcommand};
use nostr_sdk::prelude::*;
use nostr::prelude::{FromBech32, ToBech32};
use nostr::{Keys, SecretKey};
use std::time::Duration;

#[derive(Parser)]
pub struct ContactCommand {
    #[command(subcommand)]
    subcommand: ContactSubcommand,
}

#[derive(Subcommand)]
pub enum ContactSubcommand {
    /// Set contact list
    Set {
        /// Relay to send the event
        #[clap(short, long)]
        relay: String,
        /// Secret key to sign the event
        #[clap(short, long)]
        secret_key: String,
        /// Public keys to follow
        pubkeys: Vec<String>,
    },
    /// Get contact list
    Get {
        /// Relay to get the contact list from
        #[clap(short, long)]
        relay: String,
        /// Public key to get the contact list for
        pubkey: String,
    },
}

pub async fn handle_contact_command(command: ContactCommand) -> Result<(), Box<dyn std::error::Error>> {
    match command.subcommand {
        ContactSubcommand::Set { relay, secret_key, pubkeys } => {
            let secret_key = SecretKey::from_bech32(&secret_key)?;
            let keys = Keys::new(secret_key);
            let client = Client::new(keys);
            client.add_relay(&relay).await?;
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
        ContactSubcommand::Get { relay, pubkey } => {
            let pubkey = PublicKey::from_bech32(&pubkey)
                .or_else(|_| PublicKey::from_hex(&pubkey))?;

            let keys = Keys::generate();
            let client = Client::new(keys);

            let filter = Filter::new()
                .author(pubkey)
                .kind(Kind::ContactList)
                .limit(1);

            let timeout = Duration::from_secs(10);
            let events = client.fetch_events_from(vec![&relay], filter, timeout).await?;

            if let Some(event) = events.first() {
                println!("{:#?}", event.tags);
            } else {
                println!("Contact list not found.");
            }

            client.shutdown().await;
        }
    }
    Ok(())
}
