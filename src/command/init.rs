use crate::setting::{
    config, environment, get, structure, version, Language, FUSION_FRAMEWORK_VERSION,
    FUSION_TOOL_VERSION,
};

use anyhow::{bail, Context, Result};
use console::style;
use dialoguer::{Input, Select};
use std::{env, fs, path::PathBuf};

use toml;

pub fn init(
    directory: Option<String>,
    lang: Option<String>,
    name: Option<String>,
    description: Option<String>,
) -> Result<()> {
    println!();
    println!(
        "{}",
        style("Welcome to Fusion Framework!\n   MADE BY CIPHER UNIT")
            .cyan()
            .bold()
    );
    println!();

    // -----------------------------------------
    // Target Directory
    // -----------------------------------------

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

    // -----------------------------------------
    // Language
    // -----------------------------------------

    let language = match lang {
        // Non-interactive mode
        Some(lang) => parse_language(&lang)?,

        // Interactive mode
        None => {
            let languages = vec!["Python", "TypeScript / Node.js", "C#"];

            let selection = Select::new()
                .with_prompt("Select your language")
                .items(&languages)
                .default(0)
                .interact()?;

            match selection {
                0 => Language::Python,
                1 => Language::TypeScript,
                2 => Language::AspNetCore,
                _ => unreachable!(),
            }
        }
    };

    println!();

    // -----------------------------------------
    // Project Name
    // -----------------------------------------

    let default_name = target_dir
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("fusion-project")
        .to_string();

    let project_name = match name {
        Some(name) => name,

        None => Input::new()
            .with_prompt("Project name")
            .default(default_name)
            .interact_text()?,
    };

    // -----------------------------------------
    // Description
    // -----------------------------------------

    let project_description = match description {
        Some(description) => description,

        None => Input::new()
            .with_prompt("Project description")
            .default(String::from("A Fusion Framework project"))
            .interact_text()?,
    };

    // -----------------------------------------
    // Config
    // -----------------------------------------

    let config = config::Config {
        project: config::ProjectConfig {
            name: project_name,
            description: project_description,
        },

        fusionframework: config::FrameworkConfig {
            language: language.name().to_string(),
            extension: language.extension().to_string(),
            version: FUSION_FRAMEWORK_VERSION.to_string(),
        },

        tool: config::ToolConfig {
            version: FUSION_TOOL_VERSION.to_string(),
        },

        modules: vec![],
    };

    let config_content = toml::to_string_pretty(&config)?;

    let config_path = target_dir.join(get::get_toml());

    // -----------------------------------------
    // Existing Project
    // -----------------------------------------

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
            "  Language: {}",
            style(existing_config.fusionframework.language).yellow()
        );

        println!();

        version::check_version_on_system(&target_dir)?;

        return Ok(());
    }

    // -----------------------------------------
    // Create Files
    // -----------------------------------------

    fs::write(&config_path, config_content)?;

    environment::prod(&target_dir, &language)?;

    environment::stage(&target_dir, &language)?;

    environment::dev(&target_dir, &language)?;

    environment::git(&target_dir, &language)?;

    structure::create(&target_dir, &language, &config.project.name)?;

    // -----------------------------------------
    // Success
    // -----------------------------------------

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

/// Convert CLI language input into a Fusion Language
fn parse_language(value: &str) -> Result<Language> {
    match value.to_lowercase().as_str() {
        "python" | "py" => Ok(Language::Python),

        "typescript" | "ts" | "node" | "nodejs" => Ok(Language::TypeScript),

        "csharp" | "cs" | "asp-core" | "aspnet" | "aspnetcore" | "asp.net" | "asp.net-core" => {
            Ok(Language::AspNetCore)
        }

        _ => bail!(
            "Unsupported language '{}'. Available: python, typescript, csharp (asp-core)",
            value
        ),
    }
}
