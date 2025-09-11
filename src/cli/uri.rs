use clap::Parser;
use nostr_sdk::prelude::*;

#[derive(Parser, Clone)]
pub struct UriCommand {
    /// nostr: URI to parse
    uri: String,
}

pub async fn handle_uri_command(command: UriCommand) -> Result<(), Box<dyn std::error::Error>> {
    let nip21 = Nip21::parse(&command.uri)?;
    println!("{:#?}", nip21);
    Ok(())
}
