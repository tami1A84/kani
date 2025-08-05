use clap::Parser;
use nostr_sdk::prelude::*;
use nostr::nips::nip05::{Nip05Address, verify_from_raw_json};

#[derive(Parser)]
pub struct Nip05Command {
    /// Verify a NIP-05 identifier
    #[command(subcommand)]
    subcommand: Nip05Subcommand,
}

#[derive(Parser)]
pub enum Nip05Subcommand {
    Verify {
        /// NIP-05 identifier (user@example.com)
        #[clap(short, long)]
        nip05: String,
        /// Public key to verify against (bech32)
        #[clap(short, long)]
        pubkey: String,
    }
}

pub async fn handle_nip05_command(command: Nip05Command) -> Result<(), Box<dyn std::error::Error>> {
    match command.subcommand {
        Nip05Subcommand::Verify { nip05, pubkey } => {
            let pk = PublicKey::from_bech32(&pubkey)?;
            let nip05_address = Nip05Address::parse(&nip05)?;

            // Fetch the nostr.json file
            let url = nip05_address.url().to_string();
            let json_str = reqwest::get(&url).await?.text().await?;

            // Verify
            if verify_from_raw_json(&pk, &nip05_address, &json_str)? {
                println!("Verification successful!");
            } else {
                println!("Verification failed.");
            }
        }
    }
    Ok(())
}
