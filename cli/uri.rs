use clap::Parser;
use nostr_sdk::prelude::*;

#[derive(Parser)]
pub struct UriCommand {
    /// nostr: URI to parse
    uri: String,
}

pub async fn handle_uri_command(command: UriCommand) -> Result<(), Box<dyn std::error::Error>> {
    println!("URI parsing not yet implemented for this SDK version.");
    Ok(())
}
