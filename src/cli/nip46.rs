use crate::cli::CommonOptions;
use crate::config::load_config;
use clap::Parser;
use nostr::nips::nip04;
use nostr::nips::nip46::{
    NostrConnectMessage, NostrConnectMethod, NostrConnectRequest, NostrConnectURI,
};
use nostr::UnsignedEvent;
use nostr_sdk::prelude::*;
use tokio::time::{timeout, Duration};

#[derive(Parser, Clone)]
pub struct Nip46Command {
    /// Nostr Connect (NIP-46)
    #[command(subcommand)]
    subcommand: Nip46Subcommand,
    #[command(flatten)]
    common: CommonOptions,
}

#[derive(Parser, Clone)]
pub enum Nip46Subcommand {
    /// Get public key from a remote signer
    GetPublicKey {
        /// Bunker URI (nostrconnect://...)
        uri: String,
    },
    /// Sign an unsigned event using a remote signer
    SignEvent {
        /// Bunker URI (nostrconnect://...)
        uri: String,
        /// Unsigned event to sign (JSON string)
        event_json: String,
    },
}

pub async fn handle_nip46_command(command: Nip46Command) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    let secret_key_str = command
        .common
        .secret_key
        .or(config.secret_key)
        .ok_or("Secret key not provided in args or config")?;

    match command.subcommand {
        Nip46Subcommand::GetPublicKey { uri } => {
            let sk = SecretKey::from_bech32(&secret_key_str)?;
            let keys = Keys::new(sk);

            let bunker_uri = NostrConnectURI::parse(&uri)?;

            let bunker_pk =
                if let NostrConnectURI::Bunker {
                    remote_signer_public_key,
                    ..
                } = &bunker_uri
                {
                    *remote_signer_public_key
                } else {
                    return Err("Not a bunker URI".into());
                };

            let req = NostrConnectRequest::GetPublicKey;
            let msg = NostrConnectMessage::request(&req);
            let request_id = msg.id().to_string();

            let client = Client::new(keys.clone());
            for relay in bunker_uri.relays() {
                client.add_relay(relay.clone()).await?;
            }
            client.connect().await;

            let builder = EventBuilder::nostr_connect(&keys, bunker_pk, msg)?;
            let event = client.sign_event_builder(builder).await?;
            client.send_event(&event).await?;

            println!(
                "GetPublicKey request sent with id: {}",
                event.id.to_bech32()?
            );

            // Handle response
            let filter = Filter::new()
                .kind(Kind::EncryptedDirectMessage)
                .author(bunker_pk)
                .pubkey(keys.public_key())
                .since(Timestamp::now());

            client.subscribe(filter, None).await?;

            println!("Waiting for response...");

            let mut notifications = client.notifications();

            let fut = async {
                while let Ok(notification) = notifications.recv().await {
                    if let RelayPoolNotification::Event { event, .. } = notification {
                        if event.kind == Kind::EncryptedDirectMessage {
                            if let Ok(decrypted) =
                                nip04::decrypt(keys.secret_key(), &event.pubkey, &event.content)
                            {
                                if let Ok(msg) = NostrConnectMessage::from_json(&decrypted) {
                                    if msg.id() == request_id {
                                        if let Ok(response) =
                                            msg.to_response(NostrConnectMethod::GetPublicKey)
                                        {
                                            if let Some(result) = response.result {
                                                if let Ok(pk) = result.to_get_public_key() {
                                                    return Some(Ok(pk));
                                                }
                                            }
                                            if let Some(error) = response.error {
                                                return Some(Err(error));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                None
            };

            match timeout(Duration::from_secs(30), fut).await {
                Ok(Some(Ok(pk))) => println!("Received public key: {}", pk.to_bech32()?),
                Ok(Some(Err(e))) => println!("Error from bunker: {}", e),
                _ => println!("Timeout or no response."),
            }

            client.shutdown().await;
        }
        Nip46Subcommand::SignEvent { uri, event_json } => {
            let sk = SecretKey::from_bech32(&secret_key_str)?;
            let keys = Keys::new(sk);

            let bunker_uri = NostrConnectURI::parse(&uri)?;

            let bunker_pk =
                if let NostrConnectURI::Bunker {
                    remote_signer_public_key,
                    ..
                } = &bunker_uri
                {
                    *remote_signer_public_key
                } else {
                    return Err("Not a bunker URI".into());
                };

            let unsigned_event = UnsignedEvent::from_json(&event_json)?;
            let req = NostrConnectRequest::SignEvent(unsigned_event);
            let msg = NostrConnectMessage::request(&req);
            let request_id = msg.id().to_string();

            let client = Client::new(keys.clone());
            for relay in bunker_uri.relays() {
                client.add_relay(relay.clone()).await?;
            }
            client.connect().await;

            let builder = EventBuilder::nostr_connect(&keys, bunker_pk, msg)?;
            let event = client.sign_event_builder(builder).await?;
            client.send_event(&event).await?;

            println!("SignEvent request sent with id: {}", event.id.to_bech32()?);

            let filter = Filter::new()
                .kind(Kind::EncryptedDirectMessage)
                .author(bunker_pk)
                .pubkey(keys.public_key())
                .since(Timestamp::now());

            client.subscribe(filter, None).await?;

            println!("Waiting for response...");

            let mut notifications = client.notifications();

            let fut = async {
                while let Ok(notification) = notifications.recv().await {
                    if let RelayPoolNotification::Event { event, .. } = notification {
                        if event.kind == Kind::EncryptedDirectMessage {
                            if let Ok(decrypted) =
                                nip04::decrypt(keys.secret_key(), &event.pubkey, &event.content)
                            {
                                if let Ok(msg) = NostrConnectMessage::from_json(&decrypted) {
                                    if msg.id() == request_id {
                                        if let Ok(response) =
                                            msg.to_response(NostrConnectMethod::SignEvent)
                                        {
                                            if let Some(result) = response.result {
                                                if let Ok(signed_event) = result.to_sign_event() {
                                                    return Some(Ok(signed_event));
                                                }
                                            }
                                            if let Some(error) = response.error {
                                                return Some(Err(error));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                None
            };

            match timeout(Duration::from_secs(30), fut).await {
                Ok(Some(Ok(evt))) => println!("Received signed event: {}", evt.as_json()),
                Ok(Some(Err(e))) => println!("Error from bunker: {}", e),
                _ => println!("Timeout or no response."),
            }

            client.shutdown().await;
        }
    }
    Ok(())
}
