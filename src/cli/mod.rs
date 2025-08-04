use clap::{Parser, Subcommand};

pub mod key;
pub mod event;
pub mod contact;
pub mod nip19;
pub mod uri;

use self::{
    contact::ContactCommand,
    event::EventCommand,
    key::KeyCommand,
    nip19::Nip19Command,
    uri::UriCommand,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
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
    /// NIP-19 bech32 encoding/decoding
    Nip19(Nip19Command),
    /// NIP-21 nostr URI parsing
    Uri(UriCommand),
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::Key(key_command) => key::handle_key_command(key_command).await?,
        Command::Event(event_command) => event::handle_event_command(event_command).await?,
        Command::Contact(contact_command) => contact::handle_contact_command(contact_command).await?,
        Command::Nip19(nip19_command) => nip19::handle_nip19_command(nip19_command).await?,
        Command::Uri(uri_command) => uri::handle_uri_command(uri_command).await?,
    }

    Ok(())
}
