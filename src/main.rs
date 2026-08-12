/*!
 * Fusion Tool
 *
 * Copyright (c) 2026 CipherUnit
 * Licensed under the MIT License.
 *
 * Fusion Tool is the official command-line interface (CLI) for managing
 * Fusion Framework projects. It provides commands for creating, building,
 * configuring, and maintaining Fusion applications.
 *
 * If you prefer a graphical experience, use Fusion GUI, the official
 * desktop application for managing Fusion Framework projects.
 *
 * Documentation:
 *   FIXME:‌ add fusion-framework docs here!!
 *
 * GitHub:
 *   https://github.com/cipherunits/fusion-tool
 *
 * License:
 *   https://github.com/cipherunits/fusion-tool/blob/main/LICENSE
 */

mod command;
mod setting;

use anyhow::Result;
use clap::Parser;

use command::{add, exec, init, module_init, selected_env, update, Commands, ModuleCommands};

fn main() -> Result<()> {
    let cli = command::Cli::parse();

    match cli.command {

        // Initial new project
        Commands::Init {
            directory,
            lang,
            name,
            description,
        } => {
            init(directory, lang, name, description)?;
        }

        // run env commands
        // exmaple:
        // {
        //     "test": "mkdir test"
        // }
        // run test Command:
        // fusion test:dev
        Commands::Command {
            name,
            env,
            dev,
            stage,
            prod,
        } => {
            exec(name, selected_env(env, dev, stage, prod))?;
        }

        Commands::Module { command } => match command {
            ModuleCommands::Init {
                directory,
                lang,
                name,
                description,
            } => {
                module_init(directory, lang, name, description)?;
            }
        },

        Commands::Add { github } => {
            add(github)?;
        }

        // Update fusion tool with latest version
        Commands::Update => {
            update()?;
        }
    }

    Ok(())
}
