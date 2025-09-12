use crate::cli::common::{connect_client, get_relays};
use crate::cli::CommonOptions;
use crate::config::load_config;
use crate::error::Error;
use clap::{Parser, Subcommand};
use nostr::prelude::FromBech32;
use nostr::{Keys, SecretKey};
use nostr_sdk::prelude::*;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;
use std::process::Command as StdCommand;
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

async fn edit_relays(secret_key_str: String, relays: Vec<String>) -> Result<(), Error> {
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

    let mut current_relays_str = String::new();
    for (url, markers) in relay_markers {
        current_relays_str.push_str(&url);
        for marker in markers {
            current_relays_str.push_str(&format!(" #{marker}"));
        }
        current_relays_str.push('\n');
    }

    let template = r#"# Read — これによりリレーはプライベート／非表示の受信箱になります。受信箱のように機能しますが、そのように宣伝されることはありません。kaniはこのリレーであなたを参照するイベントを探します。
#   (例: wss://relay.example.com #read)
# Inbox — Readと同様ですが、他のクライアントがこのリレーにあなたをタグ付けしたイベントを送信できるように宣伝されます。これを3つか4つ持つことをお勧めします。
#   (例: wss://relay.example.com #Inbox)
# Write — これによりリレーはプライベート／非表示の送信箱になります。送信箱のように機能しますが、そのように宣伝されることはありません。kaniはここにあなたのイベントを投稿します。
#   (例: wss://relay.example.com #write)
# Outbox — Writeと同様ですが、他のクライアントがこのリレーからあなたのイベントを取得できるように宣伝されます。これを3つから5つ持つことをお勧めします。
#   (例: wss://relay.example.com #Outbox)
# Discover — この設定はリレーが他の人のリレーリストを見つけるために使われることを意味します。
#   (例: wss://relay.example.com #Discover)
# Spam Safe — 特定のスパム設定を使う場合、このリレーをスパムフィルターとして信頼することを意味し、Gossipはこのリレーから誰の返信も取得します（あなたがフォローしている人だけではなく）。
#   (例: wss://relay.example.com #Spam Safe)
# Direct Message — これは受信箱のようなものですが、DM専用です。
#   (例: wss://relay.example.com #Direct Message)
# Global feed — グローバルフィードを表示するとき、このリレーからのイベントも含まれます。選択するリレーが多いほど、グローバルフィードは賑やかになります。
#   (例: wss://relay.example.com #Global feed)
# Search — Search Relaysを実行するとき、その検索はこのリレーに送信されます。これをいくつか持つことも、非常に良いリレーを見つけた場合は1つだけ持つこともできます。
#   (例: wss://relay.example.com #Search)
#
# --- リレーの例 ---
# wss://relay.damus.io #read #write
# wss://relay.snort.social #read #write
# wss://nostr.wine #read #write
"#;

    let mut file_content = String::new();
    file_content.push_str(template);
    file_content.push('\n');
    file_content.push_str(&current_relays_str);

    // Create and write to temp file
    let mut temp_file = tempfile::NamedTempFile::new()?;
    temp_file.write_all(file_content.as_bytes())?;

    // Open editor
    let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
    let status = StdCommand::new(editor).arg(temp_file.path()).status()?;

    if !status.success() {
        return Err(Error::Message(
            "Editor exited with a non-zero status.".to_string(),
        ));
    }

    // Read from temp file
    let new_relays_str = fs::read_to_string(temp_file.path())?;

    // Parse new relays
    let mut tags = Vec::new();
    for line in new_relays_str.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut parts = line.split('#');
        let url = parts.next().unwrap().trim();
        if url.is_empty() {
            continue;
        }
        let markers: Vec<&str> = parts.map(|s| s.trim()).filter(|s| !s.is_empty()).collect();

        if markers.is_empty() {
            let tag_vec = vec!["r".to_string(), url.to_string()];
            tags.push(Tag::parse(&tag_vec)?);
        } else {
            for m in markers {
                let tag_vec = vec!["r".to_string(), url.to_string(), m.to_string()];
                tags.push(Tag::parse(&tag_vec)?);
            }
        }
    }

    // Publish new event using the same connected client
    let builder = EventBuilder::new(Kind::RelayList, "").tags(tags);
    let event = client.sign_event_builder(builder).await?;
    client.send_event(&event).await?;
    println!("Relay list updated.");

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
