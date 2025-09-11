use clap::{Parser, Subcommand};

pub mod key;
pub mod event;
pub mod contact;
pub mod nip19;
pub mod uri;
pub mod nip05;
pub mod nip46;
pub mod nip47;

use self::{
    contact::ContactCommand,
    event::EventCommand,
    key::KeyCommand,
    nip19::Nip19Command,
    uri::UriCommand,
    nip05::Nip05Command,
    nip46::Nip46Command,
    nip47::Nip47Command,
};

#[derive(Parser, Clone)]
pub struct CommonOptions {
    /// Secret key to use for signing events
    #[clap(long)]
    pub secret_key: Option<String>,

    /// Relay to connect to
    #[clap(long, short, action = clap::ArgAction::Append)]
    pub relay: Vec<String>,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Clone)]
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
    /// NIP-05 DNS-based identifiers
    Nip05(Nip05Command),
    /// NIP-46 Nostr Connect
    Nip46(Nip46Command),
    /// NIP-47 Nostr Wallet Connect
    Nip47(Nip47Command),
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::Key(key_command) => key::handle_key_command(key_command).await?,
        Command::Event(event_command) => event::handle_event_command(event_command).await?,
        Command::Contact(contact_command) => contact::handle_contact_command(contact_command).await?,
        Command::Nip19(nip19_command) => nip19::handle_nip19_command(nip19_command).await?,
        Command::Uri(uri_command) => uri::handle_uri_command(uri_command).await?,
        Command::Nip05(nip05_command) => nip05::handle_nip05_command(nip05_command).await?,
        Command::Nip46(nip46_command) => nip46::handle_nip46_command(nip46_command).await?,
        Command::Nip47(nip47_command) => nip47::handle_nip47_command(nip47_command).await?,
    }

    Ok(())
}
