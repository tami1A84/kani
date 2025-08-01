use clap::{Parser, Subcommand};
use nostr_sdk::prelude::*;
use nostr::prelude::{FromBech32, ToBech32};
use nostr::bip39::Mnemonic;
use nostr::{EventBuilder, Keys, Kind, PublicKey, Tag, SecretKey, EventId};


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Keys management
    Key(KeyCommand),
    /// Event management
    Event(EventCommand),
    /// Contact list management
    Contact(ContactCommand),
    /// OpenTimestamps
    Ots(OtsCommand),
    /// Direct Messages
    Dm(DmCommand),
    /// NIP-05
    Nip05(Nip05Command),
}

#[derive(Parser)]
struct KeyCommand {
    #[command(subcommand)]
    subcommand: KeySubcommand,
}

#[derive(Subcommand)]
enum KeySubcommand {
    /// Generate new keys
    Generate,
    /// Derives keys from a mnemonic
    FromMnemonic {
        /// Mnemonic phrase
        mnemonic: String,
    },
}

#[derive(Parser)]
struct EventCommand {
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
}

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

#[derive(Parser)]
pub struct OtsCommand {
    #[command(subcommand)]
    subcommand: OtsSubcommand,
}

#[derive(Subcommand)]
pub enum OtsSubcommand {
    /// Attest event
    Attest {
        /// Relay to send the event
        #[clap(short, long)]
        relay: String,
        /// Secret key to sign the event
        #[clap(short, long)]
        secret_key: String,
        /// Event id to attest
        event_id: String,
    },
}

#[derive(Parser)]
pub struct DmCommand {
    #[command(subcommand)]
    subcommand: DmSubcommand,
}

#[derive(Subcommand)]
pub enum DmSubcommand {
    /// Send a direct message
    Send {
        /// Relay to send the event
        #[clap(short, long)]
        relay: String,
        /// Secret key to sign the event
        #[clap(short, long)]
        secret_key: String,
        /// Receiver public key
        #[clap(long)]
        receiver: String,
        /// Message content
        message: String,
    },
    /// Receive direct messages
    Receive {
        /// Relay to get the events from
        #[clap(short, long)]
        relay: String,
        /// Secret key to decrypt the messages
        #[clap(short, long)]
        secret_key: String,
    },
}

#[derive(Parser)]
pub struct Nip05Command {
    #[command(subcommand)]
    subcommand: Nip05Subcommand,
}

#[derive(Subcommand)]
pub enum Nip05Subcommand {
    /// Verify a NIP-05 identifier
    Verify {
        /// NIP-05 identifier (user@domain)
        identifier: String,
        /// Public key to verify against
        pubkey: String,
    },
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::Key(key_command) => match key_command.subcommand {
            KeySubcommand::Generate => {
                let keys = Keys::generate();
                println!("Public key: {}", keys.public_key().to_bech32()?);
                println!("Secret key: {}", keys.secret_key()?.to_bech32()?);
            }
            KeySubcommand::FromMnemonic { mnemonic } => {
                let mnemonic = Mnemonic::parse(&mnemonic)?;
                let keys = Keys::from_mnemonic(mnemonic.to_string(), None)?;
                println!("Public key: {}", keys.public_key().to_bech32()?);
                println!("Secret key: {}", keys.secret_key()?.to_bech32()?);
            }
        },
        Command::Event(event_command) => match event_command.subcommand {
            EventSubcommand::CreateTextNote { relay, secret_key, content } => {
                let secret_key = SecretKey::from_bech32(&secret_key)?;
                let keys = Keys::new(secret_key);
                let client = Client::new(&keys);

                client.add_relay(relay).await?;
                client.connect().await;

                let event = EventBuilder::text_note(content, []).to_event(&keys)?;
                let event_id = client.send_event(event).await?;
                println!("Event sent with id: {}", event_id.to_bech32()?);

                client.shutdown().await?;
            }
            EventSubcommand::Get { relay, id } => {
                let event_id = EventId::from_bech32(&id)?;
                let client = Client::default();

                client.add_relay(relay).await?;
                client.connect().await;

                let filter = Filter::new().id(event_id);
                let events = client.get_events_of(vec![filter], None).await?;

                if let Some(event) = events.first() {
                    println!("{}", event.as_json());
                } else {
                    println!("Event not found");
                }

                client.shutdown().await?;
            }
        },
        Command::Contact(contact_command) => match contact_command.subcommand {
            ContactSubcommand::Set { relay, secret_key, pubkeys } => {
                let secret_key = SecretKey::from_bech32(&secret_key)?;
                let keys = Keys::new(secret_key);
                let client = Client::new(&keys);

                client.add_relay(relay).await?;
                client.connect().await;

                let contacts = pubkeys
                    .into_iter()
                    .map(|pk| PublicKey::from_bech32(pk).map(|pk| Contact::new(pk, None, None::<String>)))
                    .collect::<Result<Vec<_>, _>>()?;

                let event_id = client.set_contact_list(contacts).await?;
                println!("Contact list sent with id: {}", event_id.to_bech32()?);

                client.shutdown().await?;
            }
            ContactSubcommand::Get { relay, pubkey } => {
                let pubkey = PublicKey::from_bech32(&pubkey)?;
                let client = Client::default();
                
                client.add_relay(relay).await?;
                client.connect().await;

                let filter = Filter::new()
                    .author(pubkey)
                    .kind(Kind::ContactList)
                    .limit(1);
                
                let events = client.get_events_of(vec![filter], None).await?;

                if let Some(event) = events.first() {
                    println!("Found contact list for {}", pubkey.to_bech32()?);
                    for tag in event.tags() {
                        if let Tag::PublicKey { public_key, .. } = tag {
                            println!("- {}", public_key.to_bech32()?);
                        }
                    }
                } else {
                    println!("Contact list not found");
                }

                client.shutdown().await?;
            }
        },
        Command::Ots(ots_command) => match ots_command.subcommand {
            OtsSubcommand::Attest { relay, secret_key, event_id } => {
                let secret_key = SecretKey::from_bech32(&secret_key)?;
                let keys = Keys::new(secret_key);
                let client = Client::new(&keys);

                client.add_relay(relay).await?;
                client.connect().await;

                let event_id_to_attest = EventId::from_bech32(&event_id)?;

                // Dummy OTS attestation for now
                let ots = "dummy_ots_attestation_string";

                let tags = vec![
                    Tag::event(event_id_to_attest),
                    Tag::parse(&["ots", ots])?,
                ];

                let event = EventBuilder::new(Kind::Custom(1040), "", tags).to_event(&keys)?;
                let event_id = client.send_event(event).await?;
                println!("OTS Attestation sent with id: {}", event_id.to_bech32()?);

                client.shutdown().await?;
            }
        },
        Command::Dm(dm_command) => match dm_command.subcommand {
            DmSubcommand::Send { relay, secret_key, receiver, message } => {
                let secret_key = SecretKey::from_bech32(&secret_key)?;
                let keys = Keys::new(secret_key);
                let client = Client::new(&keys);

                client.add_relay(relay).await?;
                client.connect().await;

                let receiver_pubkey = PublicKey::from_bech32(&receiver)?;

                let event_id = client.send_direct_msg(receiver_pubkey, message, None).await?;
                println!("DM sent with id: {}", event_id.to_bech32()?);

                client.shutdown().await?;
            }
            DmSubcommand::Receive { relay, secret_key } => {
                let secret_key = SecretKey::from_bech32(&secret_key)?;
                let keys = Keys::new(secret_key.clone());
                let client = Client::new(&keys);

                client.add_relay(relay).await?;
                client.connect().await;

                let p_tag = nostr::SingleLetterTag::from_char('p').unwrap();
                let filter = Filter::new()
                    .kind(Kind::EncryptedDirectMessage)
                    .custom_tag(p_tag, vec![keys.public_key().to_string()]);

                println!("Listening for DMs...");
                let events = client.get_events_of(vec![filter], None).await?;
                
                for event in events {
                    if let Ok(msg) = nip04::decrypt(&secret_key, &event.pubkey, &event.content) {
                        println!("From: {}: {}", event.pubkey.to_bech32()?, msg);
                    } else {
                        eprintln!("Could not decrypt message from {}", event.pubkey.to_bech32()?);
                    }
                }

                client.shutdown().await?;
            }
        },
        Command::Nip05(nip05_command) => match nip05_command.subcommand {
            Nip05Subcommand::Verify { identifier, pubkey } => {
                let pubkey = PublicKey::from_bech32(&pubkey)?;
                
                let profile = nostr_sdk::nips::nip05::get_profile(&identifier, None).await?;
                if profile.public_key == pubkey {
                    println!("NIP-05 identifier {} is valid for public key {}", identifier, pubkey.to_bech32()?);
                } else {
                    eprintln!("NIP-05 verification failed: public key mismatch");
                }
            }
        },
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_generate_command() {
        let cli = Cli::try_parse_from(&["nostr-tool", "key", "generate"]).unwrap();
        match cli.command {
            Command::Key(key_command) => match key_command.subcommand {
                KeySubcommand::Generate => {
                    // Success
                }
                _ => panic!("Expected Generate subcommand"),
            },
            _ => panic!("Expected Key command"),
        }
    }
}
