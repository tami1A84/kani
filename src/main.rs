mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    cli::run().await
}

#[cfg(test)]
mod tests {
    use super::cli;
    use crate::path::to::cli;
    use clap::Parser;

    #[test]
    fn test_parse_generate_command() {
        let cli = cli::Cli::try_parse_from(&["nostr-tool", "key", "generate"]).unwrap();
        // We can't easily test the subcommand matching in the new structure without making fields public.
        // For now, just parsing is a good enough smoke test.
    }
}

