use clap::Parser;
use nostr_sdk::prelude::*;

#[derive(Parser, Clone)]
pub struct UriCommand {
    /// nostr: URI to parse
    uri: String,
}

use crate::error::Error;

pub async fn handle_uri_command(command: UriCommand) -> Result<(), Error> {
    let nip21 = Nip21::parse(&command.uri)?;
    println!("{nip21:#?}");
    Ok(())
}
