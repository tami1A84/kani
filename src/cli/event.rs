use clap::{Parser, Subcommand};
use nostr_sdk::prelude::*;
use nostr::prelude::{FromBech32, ToBech32};
use nostr::{Keys, SecretKey};
use nostr::EventBuilder;
use nostr_sdk::nips::nip09::EventDeletionRequest;
use std::time::Duration;

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
        EventSubcommand::Get { relay, id } => {
            let event_id = EventId::from_bech32(&id)
                .or_else(|_| EventId::from_hex(&id))?;

            let keys = Keys::generate();
            let client = Client::new(keys);

            let filter = Filter::new().id(event_id);
            let timeout = Duration::from_secs(10);
            let events = client.fetch_events_from(vec![&relay], filter, timeout).await?;

            if let Some(event) = events.first() {
                println!("{:#?}", event);
            } else {
                println!("Event not found.");
            }

            client.shutdown().await;
        }
        EventSubcommand::Delete { relay, secret_key, event_id } => {
            let secret_key = SecretKey::from_bech32(&secret_key)?;
            let keys = Keys::new(secret_key);
            let client = Client::new(keys);
            client.add_relay(&relay).await?;
            client.connect().await;

            let event_id_to_delete = EventId::from_bech32(&event_id)
                .or_else(|_| EventId::from_hex(&event_id))?;

            let request = EventDeletionRequest {
                ids: vec![event_id_to_delete],
                coordinates: vec![],
                reason: None,
            };
            let builder = EventBuilder::delete(request);
            let signed_event = client.sign_event_builder(builder).await?;
            let deletion_event_id = client.send_event(&signed_event).await?;
            println!("Deletion event sent with id: {}", deletion_event_id.to_bech32()?);

            client.shutdown().await;
        }
    }
    Ok(())
}
