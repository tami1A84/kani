use clap::{Parser, Subcommand};
use nostr_sdk::prelude::*;

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

pub async fn handle_dm_command(command: DmCommand) -> Result<(), Box<dyn std::error::Error>> {
    match command.subcommand {
        DmSubcommand::Send { .. } => {
            println!("DM Send not yet implemented for this SDK version.");
        }
        DmSubcommand::Receive { .. } => {
            println!("DM Receive not yet implemented for this SDK version.");
        }
    }
    Ok(())
}
