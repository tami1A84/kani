use crate::cli::CommonOptions;
use crate::cli::common::{connect_client, get_relays};
use crate::config::load_config;
use clap::{Parser, Subcommand};
use colored::*;
use comfy_table::{Cell, CellAlignment, Table, presets::UTF8_FULL};
use dialoguer::{Confirm, Input, theme::ColorfulTheme};
use indicatif::{ProgressBar, ProgressStyle};
use nostr::nips::nip44;
use nostr::prelude::{FromBech32, ToBech32};
use nostr::{EventBuilder, Keys, SecretKey};
use nostr_sdk::nips::nip09::EventDeletionRequest;
use nostr_sdk::prelude::*;
use serde_json;
use std::time::Duration;

#[derive(Parser, Clone)]
pub struct EventCommand {
    #[command(subcommand)]
    subcommand: EventSubcommand,
    #[command(flatten)]
    common: CommonOptions,
}

#[derive(Subcommand, Clone)]
enum EventSubcommand {
    /// Create a text note
    CreateTextNote {
        /// Text note content
        content: String,
        /// Recipient public key for gift wrap (NIP-59)
        #[clap(long)]
        gift_wrap_recipient: Option<String>,
    },
    /// Get an event by id
    Get {
        /// Event id to get
        id: String,
    },
    /// Delete an event
    Delete {
        /// ID of the event to delete
        event_id: String,
    },
    /// Encrypt a payload using NIP-44
    EncryptPayload {
        /// Recipient public key (bech32)
        #[clap(short, long)]
        recipient: String,
        /// Content to encrypt
        content: String,
    },
    /// Decrypt a payload using NIP-44
    DecryptPayload {
        /// Sender public key (bech32)
        #[clap(short, long)]
        sender: String,
        /// Encrypted content
        content: String,
    },
    /// Create a long-form content note (NIP-23)
    CreateLongFormPost {
        /// Path to the markdown file
        #[clap(short, long)]
        file: String,
        /// Title of the article
        #[clap(long)]
        title: Option<String>,
        /// Summary of the article
        #[clap(long)]
        summary: Option<String>,
        /// `d` identifier for the article
        #[clap(long)]
        d_identifier: Option<String>,
    },
    /// Edit profile metadata (NIP-01)
    EditProfile,
}

use crate::error::Error;

pub async fn handle_event_command(command: EventCommand) -> Result<(), Error> {
    let config = load_config()?;
    let relays = get_relays(&command.common, &config);

    match command.subcommand {
        EventSubcommand::CreateTextNote {
            content,
            gift_wrap_recipient,
        } => {
            let secret_key_str = command
                .common
                .secret_key
                .or(config.secret_key)
                .ok_or(Error::SecretKeyMissing)?;
            create_text_note(content, gift_wrap_recipient, secret_key_str, relays).await?;
        }
        EventSubcommand::Get { id } => {
            get_event(id, relays).await?;
        }
        EventSubcommand::Delete { event_id } => {
            let secret_key_str = command
                .common
                .secret_key
                .or(config.secret_key)
                .ok_or(Error::SecretKeyMissing)?;
            delete_event(event_id, secret_key_str, relays).await?;
        }
        EventSubcommand::EncryptPayload { recipient, content } => {
            let secret_key_str = command
                .common
                .secret_key
                .or(config.secret_key)
                .ok_or(Error::SecretKeyMissing)?;
            let sk = SecretKey::from_bech32(&secret_key_str)?;
            let pk = PublicKey::from_bech32(&recipient)?;
            let encrypted = nip44::encrypt(&sk, &pk, &content, nip44::Version::default())?;
            println!("{encrypted}");
        }
        EventSubcommand::DecryptPayload { sender, content } => {
            let secret_key_str = command
                .common
                .secret_key
                .or(config.secret_key)
                .ok_or(Error::SecretKeyMissing)?;
            let sk = SecretKey::from_bech32(&secret_key_str)?;
            let pk = PublicKey::from_bech32(&sender)?;
            let decrypted = nip44::decrypt(&sk, &pk, &content)?;
            println!("{decrypted}");
        }
        EventSubcommand::CreateLongFormPost {
            file,
            title,
            summary,
            d_identifier,
        } => {
            let secret_key_str = command
                .common
                .secret_key
                .or(config.secret_key)
                .ok_or(Error::SecretKeyMissing)?;
            create_long_form_post(file, title, summary, d_identifier, secret_key_str, relays)
                .await?;
        }
        EventSubcommand::EditProfile => {
            let secret_key_str = command
                .common
                .secret_key
                .or(config.secret_key)
                .ok_or(Error::SecretKeyMissing)?;
            edit_profile(secret_key_str, relays).await?;
        }
    }
    Ok(())
}

async fn create_text_note(
    content: String,
    gift_wrap_recipient: Option<String>,
    secret_key_str: String,
    relays: Vec<String>,
) -> Result<(), Error> {
    let secret_key = SecretKey::from_bech32(&secret_key_str)?;
    let keys = Keys::new(secret_key);
    let client = connect_client(keys.clone(), relays).await?;

    let builder = EventBuilder::text_note(&content);

    let event_to_send = if let Some(recipient_str) = gift_wrap_recipient {
        let recipient_pk = PublicKey::from_bech32(&recipient_str)?;
        let rumor = builder.build(keys.public_key());
        EventBuilder::gift_wrap(&keys, &recipient_pk, rumor, []).await?
    } else {
        client.sign_event_builder(builder).await?
    };

    let event_id = client.send_event(&event_to_send).await?;
    println!("Event sent with id: {}", event_id.to_bech32().unwrap());

    client.shutdown().await;
    Ok(())
}

pub async fn edit_profile(secret_key_str: String, relays: Vec<String>) -> Result<(), Error> {
    let keys = Keys::new(SecretKey::from_bech32(&secret_key_str)?);
    let client = connect_client(keys.clone(), relays.clone()).await?;

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.blue} Fetching existing profile...")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    spinner.enable_steady_tick(Duration::from_millis(100));

    let filter = Filter::new()
        .author(keys.public_key())
        .kind(Kind::Metadata)
        .limit(1);
    let timeout = Duration::from_secs(10);
    let relay_urls: Vec<&str> = relays.iter().map(|s| s.as_str()).collect();
    let events = client
        .fetch_events_from(relay_urls, filter, timeout)
        .await?;
    spinner.finish_and_clear();

    let mut current_metadata = if let Some(event) = events.first() {
        Metadata::from_json(&event.content)?
    } else {
        println!("No existing profile found. Creating a new one.");
        Metadata::new()
    };

    let theme = ColorfulTheme::default();

    current_metadata.name = Some(
        Input::with_theme(&theme)
            .with_prompt("Name")
            .with_initial_text(current_metadata.name.unwrap_or_default())
            .interact_text()?,
    );

    current_metadata.display_name = Some(
        Input::with_theme(&theme)
            .with_prompt("Display Name")
            .with_initial_text(current_metadata.display_name.unwrap_or_default())
            .interact_text()?,
    );

    current_metadata.about = Some(
        Input::with_theme(&theme)
            .with_prompt("About")
            .with_initial_text(current_metadata.about.unwrap_or_default())
            .interact_text()?,
    );

    current_metadata.picture = Some(
        Input::with_theme(&theme)
            .with_prompt("Picture URL")
            .with_initial_text(current_metadata.picture.unwrap_or_default())
            .interact_text()?,
    );

    current_metadata.banner = Some(
        Input::with_theme(&theme)
            .with_prompt("Banner URL")
            .with_initial_text(current_metadata.banner.unwrap_or_default())
            .interact_text()?,
    );

    current_metadata.website = Some(
        Input::with_theme(&theme)
            .with_prompt("Website URL")
            .with_initial_text(current_metadata.website.unwrap_or_default())
            .interact_text()?,
    );

    current_metadata.lud16 = Some(
        Input::with_theme(&theme)
            .with_prompt("LUD-16 (Lightning Address)")
            .with_initial_text(current_metadata.lud16.unwrap_or_default())
            .interact_text()?,
    );

    current_metadata.nip05 = Some(
        Input::with_theme(&theme)
            .with_prompt("NIP-05 Identifier")
            .with_initial_text(current_metadata.nip05.unwrap_or_default())
            .interact_text()?,
    );

    // Clean up empty strings to None
    current_metadata.name = current_metadata.name.filter(|s| !s.is_empty());
    current_metadata.display_name = current_metadata.display_name.filter(|s| !s.is_empty());
    current_metadata.about = current_metadata.about.filter(|s| !s.is_empty());
    current_metadata.picture = current_metadata.picture.filter(|s| !s.is_empty());
    current_metadata.banner = current_metadata.banner.filter(|s| !s.is_empty());
    current_metadata.website = current_metadata.website.filter(|s| !s.is_empty());
    current_metadata.lud16 = current_metadata.lud16.filter(|s| !s.is_empty());
    current_metadata.nip05 = current_metadata.nip05.filter(|s| !s.is_empty());

    let pretty_json = serde_json::to_string_pretty(&current_metadata)?;
    println!("\nNew profile metadata:\n{}", pretty_json.cyan());

    if Confirm::with_theme(&theme)
        .with_prompt("Do you want to publish this profile?")
        .default(true)
        .interact()?
    {
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.blue} Publishing profile...")
                .unwrap()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        spinner.enable_steady_tick(Duration::from_millis(100));

        let builder = EventBuilder::metadata(&current_metadata);
        let event = client.sign_event_builder(builder).await?;
        let event_id = client.send_event(&event).await?;

        spinner.finish_with_message("Published.");
        println!(
            "Profile updated with event id: {}",
            event_id.to_bech32().unwrap().green()
        );
    } else {
        println!("Profile update cancelled.");
    }

    client.shutdown().await;
    Ok(())
}

async fn create_long_form_post(
    file: String,
    title: Option<String>,
    summary: Option<String>,
    d_identifier: Option<String>,
    secret_key_str: String,
    relays: Vec<String>,
) -> Result<(), Error> {
    let keys = Keys::new(SecretKey::from_bech32(&secret_key_str)?);
    let client = connect_client(keys.clone(), relays).await?;

    let content = std::fs::read_to_string(&file)?;

    let d_tag_value = d_identifier.unwrap_or_else(|| {
        std::path::Path::new(&file)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("default-d-identifier")
            .to_string()
    });

    let mut tags: Vec<Tag> = vec![Tag::identifier(d_tag_value)];

    if let Some(title) = title {
        tags.push(Tag::parse(["title", title.as_str()])?);
    }
    if let Some(summary) = summary {
        tags.push(Tag::parse(["summary", summary.as_str()])?);
    }

    let publication_timestamp = Timestamp::now();
    let timestamp_str = publication_timestamp.as_u64().to_string();
    tags.push(Tag::parse(["published_at", &timestamp_str])?);

    let builder = EventBuilder::new(Kind::Custom(30023), &content).tags(tags);
    let event = client.sign_event_builder(builder).await?;
    let event_id = client.send_event(&event).await?;
    println!(
        "Long-form post sent with id: {}",
        event_id.to_bech32().unwrap()
    );

    client.shutdown().await;
    Ok(())
}

async fn delete_event(
    event_id_str: String,
    secret_key_str: String,
    relays: Vec<String>,
) -> Result<(), Error> {
    let keys = Keys::new(SecretKey::from_bech32(&secret_key_str)?);
    let client = connect_client(keys, relays).await?;

    let event_id_to_delete = if let Ok(id) = EventId::from_bech32(&event_id_str) {
        id
    } else {
        EventId::from_hex(&event_id_str)?
    };

    let request = EventDeletionRequest {
        ids: vec![event_id_to_delete],
        coordinates: vec![],
        reason: None,
    };
    let builder = EventBuilder::delete(request);
    let signed_event = client.sign_event_builder(builder).await?;
    let deletion_event_id = client.send_event(&signed_event).await?;
    println!(
        "Deletion event sent with id: {}",
        deletion_event_id.to_bech32().unwrap()
    );

    client.shutdown().await;
    Ok(())
}

async fn get_event(id: String, relays: Vec<String>) -> Result<(), Error> {
    if relays.is_empty() {
        return Err(Error::Message(
            "No relays provided in args or config".to_string(),
        ));
    }
    let event_id = if let Ok(id) = EventId::from_bech32(&id) {
        id
    } else {
        EventId::from_hex(&id)?
    };

    let keys = Keys::generate();
    let client = Client::new(keys);

    let relay_urls: Vec<&str> = relays.iter().map(|s| s.as_str()).collect();

    let filter = Filter::new().id(event_id);
    let timeout = Duration::from_secs(10);

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.blue} Fetching event...")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    spinner.enable_steady_tick(Duration::from_millis(100));

    let events = client
        .fetch_events_from(relay_urls, filter, timeout)
        .await?;

    spinner.finish_with_message("Done.");

    if let Some(event) = events.first() {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .set_header(vec!["Field", "Value"]);

        table.add_row(vec![
            Cell::new("Event ID".blue().bold()).set_alignment(CellAlignment::Center),
            Cell::new(event.id.to_bech32().unwrap()),
        ]);
        table.add_row(vec![
            Cell::new("Public Key".blue().bold()).set_alignment(CellAlignment::Center),
            Cell::new(event.pubkey.to_bech32().unwrap()),
        ]);
        table.add_row(vec![
            Cell::new("Kind".blue().bold()).set_alignment(CellAlignment::Center),
            Cell::new(event.kind.to_string()),
        ]);
        table.add_row(vec![
            Cell::new("Created At".blue().bold()).set_alignment(CellAlignment::Center),
            Cell::new(event.created_at.to_string()),
        ]);
        table.add_row(vec![
            Cell::new("Content".blue().bold()).set_alignment(CellAlignment::Center),
            Cell::new(&event.content),
        ]);
        let tags_str = event
            .tags
            .iter()
            .map(|t| format!("{:?}", t))
            .collect::<Vec<String>>()
            .join("\n");
        table.add_row(vec![
            Cell::new("Tags".blue().bold()).set_alignment(CellAlignment::Center),
            Cell::new(tags_str),
        ]);
        table.add_row(vec![
            Cell::new("Signature".blue().bold()).set_alignment(CellAlignment::Center),
            Cell::new(event.sig.to_string()),
        ]);

        println!("{table}");
    } else {
        println!("{}", "Event not found.".red());
    }

    Ok(())
}
