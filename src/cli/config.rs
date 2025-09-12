use clap::{Parser, Subcommand};

#[derive(Parser, Clone)]
pub struct ConfigCommand {
    #[command(subcommand)]
    command: ConfigSubCommand,
}

#[derive(Subcommand, Clone)]
enum ConfigSubCommand {
    /// Show the path to the config file
    Path,
}

use crate::error::Error;

pub async fn handle_config_command(config_command: ConfigCommand) -> Result<(), Error> {
    match config_command.command {
        ConfigSubCommand::Path => {
            let path = crate::config::get_config_path()?;
            println!("{}", path.display());
        }
    }
    Ok(())
}
