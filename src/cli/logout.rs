use crate::error::Error;
use clap::Parser;

#[derive(Parser, Clone)]
pub struct LogoutCommand {}

pub async fn handle_logout_command(_command: LogoutCommand) -> Result<(), Error> {
    println!("unset NOSTR_SECRET_KEY");
    // stderr message to the user so it doesn't get captured by eval
    eprintln!("Logout successful. Key has been cleared from your shell environment.");
    Ok(())
}
