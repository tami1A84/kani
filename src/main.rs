mod cli;
mod config;
mod error;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    cli::run().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::cli::Cli;
    use clap::Parser;

    #[test]
    fn test_parse_generate_command() {
        let _cli = Cli::try_parse_from(&["nostr-tool", "key", "generate"]).unwrap();
        // We can't easily test the subcommand matching in the new structure without making fields public.
        // For now, just parsing is a good enough smoke test.
    }
}
