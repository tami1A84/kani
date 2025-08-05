use clap::{Parser, Subcommand};
use nostr_sdk::prelude::*;
use nostr::prelude::{FromBech32, ToBech32};
use nostr::{Keys, SecretKey};
use nostr::EventBuilder;
use nostr::nips::{nip04, nip44};
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
        /// Recipient public key for gift wrap (NIP-59)
        #[clap(long)]
        gift_wrap_recipient: Option<String>,
    },
    /// Create a direct message (NIP-04)
    CreateDm {
        /// Relay to send the event
        #[clap(short, long)]
        relay: String,
        /// Secret key to sign the event
        #[clap(short, long)]
        secret_key: String,
        /// Recipient public key (bech32)
        #[clap(short, long)]
        recipient: String,
        /// DM content
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
    /// Encrypt a payload using NIP-44
    EncryptPayload {
        /// Secret key to use for encryption (bech32)
        #[clap(short, long)]
        secret_key: String,
        /// Recipient public key (bech32)
        #[clap(short, long)]
        recipient: String,
        /// Content to encrypt
        content: String,
    },
    /// Decrypt a payload using NIP-44
    DecryptPayload {
        /// Secret key to use for decryption (bech32)
        #[clap(short, long)]
        secret_key: String,
        /// Sender public key (bech32)
        #[clap(short, long)]
        sender: String,
        /// Encrypted content
        content: String,
    },
}

pub async fn handle_event_command(command: EventCommand) -> Result<(), Box<dyn std::error::Error>> {
    match command.subcommand {
        EventSubcommand::CreateTextNote { relay, secret_key, content, gift_wrap_recipient } => {
            let secret_key = SecretKey::from_bech32(&secret_key)?;
            let keys = Keys::new(secret_key);
            let client = Client::new(keys.clone());

            client.add_relay(&relay).await?;
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
        EventSubcommand::CreateDm { relay, secret_key, recipient, content } => {
            let secret_key = SecretKey::from_bech32(&secret_key)?;
            let keys = Keys::new(secret_key);
            let recipient_pubkey = PublicKey::from_bech32(&recipient)?;

            let client = Client::new(keys.clone());
            client.add_relay(&relay).await?;
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
        EventSubcommand::EncryptPayload { secret_key, recipient, content } => {
            let sk = SecretKey::from_bech32(&secret_key)?;
            let pk = PublicKey::from_bech32(&recipient)?;
            let encrypted = nip44::encrypt(&sk, &pk, &content, nip44::Version::default())?;
            println!("{}", encrypted);
        }
        EventSubcommand::DecryptPayload { secret_key, sender, content } => {
            let sk = SecretKey::from_bech32(&secret_key)?;
            let pk = PublicKey::from_bech32(&sender)?;
            let decrypted = nip44::decrypt(&sk, &pk, &content)?;
            println!("{}", decrypted);
        }
    }
    Ok(())
}
