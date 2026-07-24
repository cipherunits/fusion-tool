use crate::setting::*;
use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use console::style;
use dialoguer::{Input, Select};
use std::{
    env,
    fs::{self},
    path::PathBuf,
};

use toml;

#[derive(Parser)]
#[command(
    name = "fusion",
    version = FUSION_TOOL_VERSION,
    about = "Fusion Framework CLI"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new Fusion Framework project
    Init {
        /// Optional project directory
        directory: Option<String>,
    },
}

pub fn init(directory: Option<String>) -> Result<()> {
    println!();
    println!(
        "{}",
        style("Welcome to Fusion Framework!\n   MADE BY‌‌ CIPHER UNIT")
            .cyan()
            .bold()
    );
    println!();

    let target_dir = match directory {
        Some(directory) => {
            let path = PathBuf::from(directory);

            if path.exists() {
                bail!("Directory '{}' already exists.", path.display());
            }

            fs::create_dir_all(&path)?;

            path
        }

        None => env::current_dir().context("Could not determine current directory")?,
    };

    let languages = vec!["Python", "TypeScript (soon)", "ASP.NET Core (soon)"];

    let selection = Select::new()
        .with_prompt("Select your language")
        .items(&languages)
        .default(0)
        .interact()?;

    let language = match selection {
        0 => Language::Python,
        1 => Language::TypeScript,
        2 => Language::AspNetCore,
        _ => unreachable!(),
    };

    println!();

    let default_name = target_dir
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("fusion-project")
        .to_string();

    let project_name: String = Input::new()
        .with_prompt("Project name")
        .default(default_name)
        .interact_text()?;

    let description: String = Input::new()
        .with_prompt("Project description")
        .default(String::from("A Fusion Framework project"))
        .interact_text()?;

    let config = config::Config {
        project: config::ProjectConfig {
            name: project_name,
            description,
        },

        fusionframework: config::FrameworkConfig {
            language: language.name().to_string(),
            extension: language.extension().to_string(),
            version: config::FUSION_FRAMEWORK_VERSION.to_string(),
        },

        tool: config::ToolConfig {
            version: config::FUSION_TOOL_VERSION.to_string(),
        },
    };

    let config_content = toml::to_string_pretty(&config)?;

    let config_path = target_dir.join(get::get_toml());

    if config_path.exists() {
        let existing_content = fs::read_to_string(&config_path)?;
        let existing_config: config::Config = toml::from_str(&existing_content)?;

        println!();
        println!(
            "{}",
            style(format!(
                "Fusion Framework v{} already exists on system.",
                existing_config.fusionframework.version
            ))
            .red()
            .bold()
        );
        println!(
            "  Current version in toml: {}",
            style(&existing_config.fusionframework.version).yellow()
        );
        println!("  Tool version: {}", style(FUSION_TOOL_VERSION).yellow());
        println!(
            "  language: {}",
            style(existing_config.fusionframework.language).yellow()
        );
        println!();

        return Ok(());
    }

    fs::write(&config_path, config_content)?;

    environment::prod().unwrap();
    environment::stage().unwrap();
    environment::dev().unwrap();

    environment::git().unwrap();

    println!();
    println!(
        "{}",
        style("✔ Project created successfully!").green().bold()
    );

    println!();
    println!("  Language: {}", style(language.name()).yellow());

    println!("  Config: {}", style(config_path.display()).yellow());

    println!();

    Ok(())
}
