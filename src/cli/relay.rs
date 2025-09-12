use crate::cli::common::{connect_client, get_relays};
use crate::cli::CommonOptions;
use crate::config::load_config;
use crate::error::Error;
use clap::{Parser, Subcommand};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};
use nostr::prelude::FromBech32;
use nostr::{Keys, SecretKey};
use nostr_sdk::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Parser, Clone)]
pub struct RelayCommand {
    #[command(subcommand)]
    subcommand: RelaySubcommand,
    #[command(flatten)]
    common: CommonOptions,
}

#[derive(Subcommand, Clone)]
pub enum RelaySubcommand {
    /// Set relay list (NIP-65)
    Set {
        /// Relays to include in the list. Format: wss://relay.example.com[#read|#write]
        relays: Vec<String>,
    },
    /// Get relay list (NIP-65)
    Get {
        /// Public key
        #[clap(short, long)]
        pubkey: String,
    },
    /// Edit relay list (NIP-65) in your editor
    Edit,
}

pub async fn handle_relay_command(command: RelayCommand) -> Result<(), Error> {
    let config = load_config()?;
    let relays = get_relays(&command.common, &config);

    match command.subcommand {
        RelaySubcommand::Set {
            relays: relays_to_set,
        } => {
            let secret_key_str = command
                .common
                .secret_key
                .or(config.secret_key)
                .ok_or(Error::SecretKeyMissing)?;
            set_relays(relays_to_set, secret_key_str, relays).await?;
        }
        RelaySubcommand::Get { pubkey } => {
            get_relays_list(pubkey, relays).await?;
        }
        RelaySubcommand::Edit => {
            let secret_key_str = command
                .common
                .secret_key
                .or(config.secret_key)
                .ok_or(Error::SecretKeyMissing)?;
            edit_relays(secret_key_str, relays).await?;
        }
    }
    Ok(())
}

pub async fn edit_relays(secret_key_str: String, relays: Vec<String>) -> Result<(), Error> {
    let keys = Keys::new(SecretKey::from_bech32(&secret_key_str)?);
    let client = connect_client(keys.clone(), relays.clone()).await?;

    // Fetch existing relay list
    let filter = Filter::new()
        .author(keys.public_key())
        .kind(Kind::RelayList)
        .limit(1);
    let timeout = Duration::from_secs(10);
    let relay_urls: Vec<&str> = relays.iter().map(|s| s.as_str()).collect();
    let events = client
        .fetch_events_from(relay_urls, filter, timeout)
        .await?;

    let mut relay_markers: HashMap<String, Vec<String>> = HashMap::new();
    if let Some(event) = events.first() {
        for tag in event.tags.iter() {
            let tag_vec = tag.clone().to_vec();
            if tag_vec.first().map(|s| s.as_str()) == Some("r") {
                if let Some(url) = tag_vec.get(1) {
                    let markers = relay_markers.entry(url.clone()).or_default();
                    if let Some(marker) = tag_vec.get(2) {
                        markers.push(marker.clone());
                    }
                }
            }
        }
    }

    let theme = ColorfulTheme::default();

    loop {
        let mut items: Vec<String> = relay_markers
            .keys()
            .map(|k| {
                let markers = relay_markers.get(k).unwrap();
                if markers.is_empty() {
                    k.clone()
                } else {
                    format!("{} (#{})", k, markers.join(", #"))
                }
            })
            .collect();
        items.sort();

        let selection = Select::with_theme(&theme)
            .with_prompt("リレーリストの編集")
            .items(&items)
            .item("リレーの追加")
            .item("リレーの削除")
            .item("完了")
            .default(0)
            .interact()?;

        match selection {
            i if i < items.len() => {
                // Edit existing relay
                let url_to_edit = &items[i].split(' ').next().unwrap().to_string();
                let new_url: String = Input::with_theme(&theme)
                    .with_prompt("リレーURL")
                    .with_initial_text(url_to_edit)
                    .validate_with(|input: &String| -> Result<(), &str> {
                        if input.starts_with("wss://") {
                            Ok(())
                        } else {
                            Err("リレーURLはwss://で始まる必要があります。")
                        }
                    })
                    .interact_text()?;

                let marker_options = &[
                    "read",
                    "write",
                    "Inbox",
                    "Outbox",
                    "Discover",
                    "Spam Safe",
                    "Direct Message",
                    "Global feed",
                    "Search",
                ];
                let existing_markers = relay_markers.get(url_to_edit).unwrap();
                let initial_selection: Vec<bool> = marker_options
                    .iter()
                    .map(|&option| existing_markers.iter().any(|m| m == option))
                    .collect();

                let new_markers_indices = MultiSelect::with_theme(&theme)
                    .with_prompt("マーカーの選択 (read/write)")
                    .items(marker_options)
                    .defaults(&initial_selection)
                    .interact()?;

                let new_markers = new_markers_indices
                    .iter()
                    .map(|&i| marker_options[i].to_string())
                    .collect();

                if &new_url != url_to_edit {
                    relay_markers.remove(url_to_edit);
                }
                relay_markers.insert(new_url, new_markers);
            }
            i if i == items.len() => {
                // Add new relay
                let url: String = Input::with_theme(&theme)
                    .with_prompt("新しいリレーURL")
                    .validate_with(|input: &String| -> Result<(), &str> {
                        if input.starts_with("wss://") {
                            Ok(())
                        } else {
                            Err("リレーURLはwss://で始まる必要があります。")
                        }
                    })
                    .interact_text()?;
                if url.is_empty() {
                    continue;
                }

                let marker_options = &[
                    "read",
                    "write",
                    "Inbox",
                    "Outbox",
                    "Discover",
                    "Spam Safe",
                    "Direct Message",
                    "Global feed",
                    "Search",
                ];
                let markers_indices = MultiSelect::with_theme(&theme)
                    .with_prompt("マーカーの選択 (read/write)")
                    .items(marker_options)
                    .interact()?;
                let markers = markers_indices
                    .iter()
                    .map(|&i| marker_options[i].to_string())
                    .collect();
                relay_markers.insert(url, markers);
            }
            i if i == items.len() + 1 => {
                // Delete relay
                if items.is_empty() {
                    println!("削除するリレーがありません。");
                    continue;
                }
                let to_delete_idx = Select::with_theme(&theme)
                    .with_prompt("削除するリレーを選択")
                    .items(&items)
                    .interact()?;
                let url_to_delete = &items[to_delete_idx].split(' ').next().unwrap().to_string();
                if Confirm::with_theme(&theme)
                    .with_prompt(format!("本当に {} を削除しますか?", url_to_delete))
                    .interact()?
                {
                    relay_markers.remove(url_to_delete);
                    println!("リレー {} を削除しました。", url_to_delete);
                }
            }
            _ => {
                // Finish
                break;
            }
        }
    }

    let mut tags = Vec::new();
    for (url, markers) in relay_markers {
        if markers.is_empty() {
            let tag_vec = vec!["r".to_string(), url.to_string()];
            tags.push(Tag::parse(&tag_vec)?);
        } else {
            for m in markers {
                let tag_vec = vec!["r".to_string(), url.to_string(), m];
                tags.push(Tag::parse(&tag_vec)?);
            }
        }
    }

    // Publish new event using the same connected client
    let builder = EventBuilder::new(Kind::RelayList, "").tags(tags);
    let event = client.sign_event_builder(builder).await?;
    client.send_event(&event).await?;
    println!("リレーリストが更新されました。");

    client.shutdown().await;
    Ok(())
}

async fn get_relays_list(pubkey: String, relays: Vec<String>) -> Result<(), Error> {
    if relays.is_empty() {
        return Err(Error::Message(
            "No relays provided in args or config".to_string(),
        ));
    }
    let pubkey = if let Ok(pk) = PublicKey::from_bech32(&pubkey) {
        pk
    } else {
        PublicKey::from_hex(&pubkey)?
    };

    let keys = Keys::generate();
    let client = Client::new(keys);

    let relay_urls: Vec<&str> = relays.iter().map(|s| s.as_str()).collect();

    let filter = Filter::new().author(pubkey).kind(Kind::RelayList).limit(1);

    let timeout = Duration::from_secs(10);
    let events = client
        .fetch_events_from(relay_urls, filter, timeout)
        .await?;

    if let Some(event) = events.first() {
        println!("{:#?}", event.tags);
    } else {
        println!("Relay list not found.");
    }

    client.shutdown().await;
    Ok(())
}

async fn set_relays(
    relays_to_set: Vec<String>,
    secret_key_str: String,
    relays: Vec<String>,
) -> Result<(), Error> {
    let keys = Keys::new(SecretKey::from_bech32(&secret_key_str)?);
    let client = connect_client(keys, relays).await?;

    let mut tags = Vec::new();
    for r in relays_to_set {
        let mut parts = r.splitn(2, '#');
        let url = parts.next().unwrap();
        let marker = parts.next();

        let tag_vec = if let Some(m) = marker {
            if m == "read" || m == "write" {
                vec!["r".to_string(), url.to_string(), m.to_string()]
            } else {
                vec!["r".to_string(), url.to_string()]
            }
        } else {
            vec!["r".to_string(), url.to_string()]
        };
        tags.push(Tag::parse(&tag_vec)?);
    }

    let builder = EventBuilder::new(Kind::RelayList, "").tags(tags);
    let event = client.sign_event_builder(builder).await?;
    client.send_event(&event).await?;
    println!("Relay list updated.");

    client.shutdown().await;
    Ok(())
}
