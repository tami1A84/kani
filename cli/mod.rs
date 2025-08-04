use clap::{Parser, Subcommand};

pub mod key;
pub mod event;
pub mod contact;
pub mod ots;
pub mod dm;
pub mod nip05;
pub mod relay;
pub mod nip19;
pub mod uri;
pub mod connect;

use self::{
    connect::ConnectCommand,
    contact::ContactCommand,
    dm::DmCommand,
    event::EventCommand,
    key::KeyCommand,
    nip05::Nip05Command,
    nip19::Nip19Command,
    ots::OtsCommand,
    relay::RelayCommand,
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
    /// OpenTimestamps
    Ots(OtsCommand),
    /// Direct Messages
    Dm(DmCommand),
    /// NIP-05
    Nip05(Nip05Command),
    /// Relay management
    Relay(RelayCommand),
    /// NIP-19 bech32 encoding/decoding
    Nip19(Nip19Command),
    /// NIP-21 nostr URI parsing
    Uri(UriCommand),
    /// NIP-46 Nostr Connect
    Connect(ConnectCommand),
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::Key(key_command) => key::handle_key_command(key_command).await?,
        Command::Event(event_command) => event::handle_event_command(event_command).await?,
        Command::Contact(contact_command) => contact::handle_contact_command(contact_command).await?,
        Command::Ots(ots_command) => ots::handle_ots_command(ots_command).await?,
        Command::Dm(dm_command) => dm::handle_dm_command(dm_command).await?,
        Command::Nip05(nip05_command) => nip05::handle_nip05_command(nip05_command).await?,
        Command::Relay(relay_command) => relay::handle_relay_command(relay_command).await?,
        Command::Nip19(nip19_command) => nip19::handle_nip19_command(nip19_command).await?,
        Command::Uri(uri_command) => uri::handle_uri_command(uri_command).await?,
        Command::Connect(connect_command) => connect::handle_connect_command(connect_command).await?,
    }

    Ok(())
}
