use crate::setting::{
    config, environment, get, structure, version, Language, FUSION_FRAMEWORK_VERSION,
    FUSION_TOOL_VERSION,
};

use anyhow::{bail, Context, Result};
use console::{style, Term};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal,
};
use dialoguer::Input;
use std::io;
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
        None => select_language()?,
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

fn select_language() -> Result<Language> {
    const ITEMS: [&str; 3] = ["Python", "TypeScript / Node.js", "C#"];

    let term = Term::stdout();
    if !term.is_term() {
        bail!(
            "Interactive language selection requires a terminal. \
             Re-run with --lang python|typescript|csharp, or run `fusion init` in an interactive terminal."
        );
    }

    // Raw mode so arrow keys arrive immediately and unescaped. `crossterm` parses
    // both CSI (`ESC [ A/B`) and SS3 application-cursor (`ESC O A/B`) sequences,
    // which `console`/`dialoguer` ignore — that is why the highlight would not move
    // in some terminals. Raw mode (and the cursor) are restored on drop.
    terminal::enable_raw_mode()?;
    struct ModeGuard;
    impl Drop for ModeGuard {
        fn drop(&mut self) {
            let _ = terminal::disable_raw_mode();
        }
    }
    let _mode_guard = ModeGuard;

    struct CursorGuard<'a>(&'a Term);
    impl Drop for CursorGuard<'_> {
        fn drop(&mut self) {
            let _ = self.0.show_cursor();
        }
    }
    let _cursor_guard = CursorGuard(&term);
    term.hide_cursor()?;

    let render = |term: &Term, sel: usize| -> io::Result<()> {
        for (i, item) in ITEMS.iter().enumerate() {
            term.clear_line()?;
            if i == sel {
                term.write_str(&format!(
                    "{} {}\r\n",
                    style(">").green().bold(),
                    style(item).bold()
                ))?;
            } else {
                term.write_str(&format!("  {}\r\n", item))?;
            }
        }
        Ok(())
    };

    let mut sel: usize = 0;
    term.write_line("Select your language:")?;
    render(&term, sel)?;

    loop {
        match event::read()? {
            Event::Key(key) => match key.code {
                KeyCode::Down | KeyCode::Char('j') => {
                    if sel + 1 < ITEMS.len() {
                        sel += 1;
                    }
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if sel > 0 {
                        sel -= 1;
                    }
                }
                KeyCode::Enter | KeyCode::Char(' ') => break,
                KeyCode::Char(c) if c >= '1' && c <= '3' => {
                    sel = (c as u8 - b'1') as usize;
                    break;
                }
                _ => {}
            },
            _ => {}
        }

        term.move_cursor_up(ITEMS.len())?;
        render(&term, sel)?;
    }

    // Erase the menu block (header + items) and print a single confirmation line.
    let total = ITEMS.len() + 1;
    term.move_cursor_up(total)?;
    for _ in 0..total {
        term.clear_line()?;
        term.move_cursor_down(1)?;
    }
    term.move_cursor_up(total)?;
    term.write_line(&format!("Select your language: {}", style(ITEMS[sel]).bold()))?;

    match sel {
        0 => Ok(Language::Python),
        1 => Ok(Language::TypeScript),
        2 => Ok(Language::AspNetCore),
        _ => unreachable!(),
    }
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
