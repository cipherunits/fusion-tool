mod command;
mod setting;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = command::Cli::parse();

    match cli.command {
        command::Commands::Init { directory } => {
            command::init(directory)?;
        }
    }

    Ok(())
}
