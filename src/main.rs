mod command;
mod setting;

use anyhow::Result;
use clap::Parser;

use command::{init, Commands};

fn main() -> Result<()> {
    let cli = command::Cli::parse();

    match cli.command {
        Commands::Init {
            directory,
            lang,
            name,
            description,
        } => {
            init(directory, lang, name, description)?;
        }
    }

    Ok(())
}