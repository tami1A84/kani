use clap::{Parser, Subcommand};
use nostr_sdk::prelude::*;

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
        ContactSubcommand::Set { .. } => {
            println!("Contact Set not yet implemented for this SDK version.");
        }
        ContactSubcommand::Get { .. } => {
            println!("Contact Get not yet implemented for this SDK version.");
        }
    }
    Ok(())
}
