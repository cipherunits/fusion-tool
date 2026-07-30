mod command;
mod setting;

use anyhow::Result;
use clap::Parser;

use command::{exec, init, selected_env, update, Commands};

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

        Commands::Command {
            name,
            env,
            dev,
            stage,
            prod,
        } => {
            exec(name, selected_env(env, dev, stage, prod))?;
        }

        Commands::Update => {
            update()?;
        }
    }

    Ok(())
}
