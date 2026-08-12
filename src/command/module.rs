use crate::setting::module::{
    is_valid_id, scaffold, ModuleImplLanguage, ModuleInitOptions,
};
use anyhow::{bail, Context, Result};
use console::style;
use dialoguer::{Confirm, Input, MultiSelect, Select};
use std::{env, fs, path::PathBuf};

pub fn module_init(
    directory: Option<String>,
    lang: Option<String>,
    name: Option<String>,
    description: Option<String>,
) -> Result<()> {
    println!();
    println!(
        "{}",
        style("Fusion Module — publishable package scaffold")
            .cyan()
            .bold()
    );
    println!();

    let lang_from_cli = lang.is_some();

    let language = match lang {
        Some(value) => ModuleImplLanguage::parse(&value)?,
        None => {
            let languages = vec![
                "Python — pure Fusion module",
                "TypeScript — pure Fusion module",
                "Rust — core + PyO3 / N-API (all host languages)",
            ];

            let selection = Select::new()
                .with_prompt("Module implementation language")
                .items(&languages)
                .default(2)
                .interact()?;

            match selection {
                0 => ModuleImplLanguage::Python,
                1 => ModuleImplLanguage::TypeScript,
                2 => ModuleImplLanguage::Rust,
                _ => unreachable!(),
            }
        }
    };

    let (target_python, target_typescript) = match language {
        ModuleImplLanguage::Python => (true, false),
        ModuleImplLanguage::TypeScript => (false, true),
        ModuleImplLanguage::Rust if lang_from_cli => (true, true),
        ModuleImplLanguage::Rust => prompt_rust_targets()?,
    };

    let module_name: String = match name {
        Some(name) => name,
        None => Input::new()
            .with_prompt("Module name (id)")
            .default("example".into())
            .interact_text()?,
    };

    let id = module_name.to_lowercase().replace('_', "-");

    if !is_valid_id(&id) {
        bail!(
            "Invalid module name '{}'. Use lowercase letters, digits, and hyphens.",
            module_name
        );
    }

    let module_description: String = match description {
        Some(description) => description,
        None => Input::new()
            .with_prompt("Description")
            .default(format!("Fusion module {id}"))
            .interact_text()?,
    };

    let target_dir = match directory {
        Some(directory) => PathBuf::from(directory),
        None => {
            let default = format!("fusion-{id}-mod");
            let chosen: String = Input::new()
                .with_prompt("Output directory")
                .default(default)
                .interact_text()?;
            PathBuf::from(chosen)
        }
    };

    let target_dir = if target_dir.is_absolute() {
        target_dir
    } else {
        env::current_dir()?.join(target_dir)
    };

    if target_dir.exists() {
        let is_empty = fs::read_dir(&target_dir)
            .with_context(|| format!("Could not read {}", target_dir.display()))?
            .next()
            .is_none();

        if !is_empty {
            let overwrite = Confirm::new()
                .with_prompt(format!(
                    "{} exists and is not empty. Abort instead of overwriting?",
                    target_dir.display()
                ))
                .default(true)
                .interact()?;

            if overwrite {
                bail!("Aborted. Choose an empty directory.");
            }

            bail!(
                "Refusing to write into a non-empty directory: {}",
                target_dir.display()
            );
        }
    }

    scaffold(
        &target_dir,
        &ModuleInitOptions {
            name: id.clone(),
            description: module_description,
            language,
            target_python,
            target_typescript,
        },
    )?;

    println!();
    println!(
        "{}",
        style("✔ Module package created successfully!")
            .green()
            .bold()
    );
    println!();
    let suggested_package = match language {
        ModuleImplLanguage::TypeScript => {
            crate::setting::module::manifest::npm_package_name(&id)
        }
        _ => crate::setting::module::manifest::python_package_name(&id),
    };

    println!("  Path:     {}", style(target_dir.display()).yellow());
    println!("  Language: {}", style(language.as_str()).yellow());
    println!("  Manifest: {}", style("fusion.module.toml").yellow());
    println!(
        "  Package:  {} {}",
        style(&suggested_package).yellow(),
        style("(recommended fusion_<name>_mod / fusion-<name>-mod)").dim()
    );
    println!();
    println!("Next:");
    println!("  1. Implement your package API");
    println!("  2. Push to GitHub");
    println!(
        "  3. In an app: {}",
        style(format!("fusion add --github YOU/fusion-{id}-mod")).cyan()
    );
    println!("  4. Import the package in your Fusion code");
    println!();

    Ok(())
}

fn prompt_rust_targets() -> Result<(bool, bool)> {
    let items = vec!["Python (PyO3)", "TypeScript (N-API)"];
    let defaults = vec![true, true];

    let selected = MultiSelect::new()
        .with_prompt("Host languages this Rust module should support")
        .items(&items)
        .defaults(&defaults)
        .interact()?;

    let target_python = selected.contains(&0);
    let target_typescript = selected.contains(&1);

    if !target_python && !target_typescript {
        bail!("Select at least one host language target.");
    }

    Ok((target_python, target_typescript))
}
