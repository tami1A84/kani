use clap::Parser;
use nostr_sdk::prelude::*;

#[derive(Parser)]
pub struct ConnectCommand {
    /// Relay where the connection will be established
    #[clap(short, long)]
    relay: Url,
    /// Name of the client application that will connect
    #[clap(short, long)]
    client_name: String,
}

pub async fn handle_connect_command(command: ConnectCommand) -> Result<(), Box<dyn std::error::Error>> {
    println!("NIP-46 Connect not yet implemented for this SDK version.");
    Ok(())
}
