use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Nostr SDK client error: {0}")]
    NostrSdkClient(#[from] nostr_sdk::client::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("TOML Deserialization error: {0}")]
    TomlDe(#[from] toml::de::Error),

    #[error("TOML Serialization error: {0}")]
    TomlSer(#[from] toml::ser::Error),

    #[error("Bech32 error: {0}")]
    Bech32(#[from] bech32::DecodeError),

    #[error("Event error: {0}")]
    Event(#[from] nostr::event::Error),

    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("nostr-sdk URL parse error: {0}")]
    NostrSdkUrlParse(#[from] nostr_sdk::types::url::Error),

    #[error("Key error: {0}")]
    Key(#[from] nostr_sdk::key::Error),

    #[error("Event builder error: {0}")]
    EventBuilder(#[from] nostr_sdk::event::builder::Error),

    #[error("BIP-39 error: {0}")]
    Bip39(#[from] nostr::bip39::Error),

    #[error("NIP-06 error: {0}")]
    Nip06(#[from] nostr::nips::nip06::Error),

    #[error("NIP-19 error: {0}")]
    Nip19(#[from] nostr::nips::nip19::Error),

    #[error("NIP-21 error: {0}")]
    Nip21(#[from] nostr_sdk::nips::nip21::Error),

    #[error("NIP-05 error: {0}")]
    Nip05(#[from] nostr::nips::nip05::Error),

    #[error("NIP-46 error: {0}")]
    Nip46(#[from] nostr::nips::nip46::Error),

    #[error("NIP-47 error: {0}")]
    Nip47(#[from] nostr::nips::nip47::Error),

    #[error("NIP-04 error: {0}")]
    Nip04(#[from] nostr::nips::nip04::Error),

    #[error("NIP-44 error: {0}")]
    Nip44(#[from] nostr::nips::nip44::Error),

    #[error("NIP-49 error: {0}")]
    Nip49(#[from] nostr::nips::nip49::Error),

    #[error("NIP-59 error: {0}")]
    Nip59(#[from] nostr::nips::nip59::Error),

    #[error("Tag error: {0}")]
    Tag(#[from] nostr_sdk::event::tag::Error),

    #[error("Secret key is missing")]
    SecretKeyMissing,

    #[error("{0}")]
    Message(String),
}
