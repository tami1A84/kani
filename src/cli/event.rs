use clap::{Parser, Subcommand};
use nostr_sdk::prelude::*;
use nostr::prelude::{FromBech32, ToBech32};
use nostr::{Keys, SecretKey};

#[derive(Parser)]
pub struct EventCommand {
    #[command(subcommand)]
    subcommand: EventSubcommand,
}

#[derive(Subcommand)]
enum EventSubcommand {
    /// Create a text note
    CreateTextNote {
        /// Relay to send the event
        #[clap(short, long)]
        relay: String,
        /// Secret key to sign the event
        #[clap(short, long)]
        secret_key: String,
        /// Text note content
        content: String,
    },
    /// Get an event by id
    Get {
        /// Relay to get the event from
        #[clap(short, long)]
        relay: String,
        /// Event id to get
        id: String,
    },
    /// Delete an event
    Delete {
        /// Relay to publish the deletion event to
        #[clap(short, long)]
        relay: String,
        /// Secret key to sign the event
        #[clap(short, long)]
        secret_key: String,
        /// ID of the event to delete
        event_id: String,
    },
}

use nostr::EventBuilder;

pub async fn handle_event_command(command: EventCommand) -> Result<(), Box<dyn std::error::Error>> {
    match command.subcommand {
        EventSubcommand::CreateTextNote { relay, secret_key, content } => {
            let secret_key = SecretKey::from_bech32(&secret_key)?;
            let keys = Keys::new(secret_key);
            let client = Client::new(keys);

            client.add_relay(&relay).await?;
            client.connect().await;

            let builder = EventBuilder::text_note(&content);
            let event = client.sign_event_builder(builder).await?;
            let event_id = client.send_event(&event).await?;
            println!("Event sent with id: {}", event_id.to_bech32()?);

            client.shutdown().await;
        }
        EventSubcommand::Get { .. } => {
            println!("Event Get not yet implemented for this SDK version.");
        }
        EventSubcommand::Delete { .. } => {
            println!("Event Delete not yet implemented for this SDK version.");
        }
    }
    Ok(())
}
