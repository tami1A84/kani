use clap::{Parser, Subcommand};
use nostr_sdk::prelude::*;

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

pub async fn handle_ots_command(command: OtsCommand) -> Result<(), Box<dyn std::error::Error>> {
    match command.subcommand {
        OtsSubcommand::Attest { .. } => {
            println!("OTS Attest not yet implemented for this SDK version.");
        }
    }
    Ok(())
}
