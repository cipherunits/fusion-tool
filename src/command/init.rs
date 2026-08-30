use crate::setting::{
    config, environment, get, structure, version, Language, FUSION_FRAMEWORK_VERSION,
    FUSION_TOOL_VERSION,
};

use anyhow::{bail, Context, Result};
use console::{style, Term};
use dialoguer::Input;
use libc;
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

/// Navigation actions understood by the interactive language picker.
enum Nav {
    Up,
    Down,
    Confirm,
    Number(usize),
    Ignore,
}

/// Read one byte from `fd`, waiting up to `timeout_ms` (negative blocks forever).
fn poll_byte(fd: i32, timeout_ms: i32) -> io::Result<Option<u8>> {
    let mut pfd = libc::pollfd {
        fd,
        events: libc::POLLIN,
        revents: 0,
    };
    let n = unsafe { libc::poll(&mut pfd as *mut _, 1, timeout_ms) };
    if n <= 0 {
        return Ok(None);
    }
    let mut buf = [0u8; 1];
    let r = unsafe { libc::read(fd, buf.as_mut_ptr() as *mut _, 1) };
    if r <= 0 {
        Ok(None)
    } else {
        Ok(Some(buf[0]))
    }
}

/// Read a single navigation key in raw mode.
///
/// `console` only parses the CSI arrow form (`ESC [ A/B`); many terminals emit the
/// SS3 application-cursor form (`ESC O A/B`) which `console` returns as an opaque
/// `UnknownEscSeq`, so `dialoguer::Select` silently ignores it and the highlight
/// never moves. We read the raw escape sequence ourselves and accept both forms,
/// plus `j`/`k` (vi keys) and number keys `1`–`3`.
fn read_nav(fd: i32) -> io::Result<Nav> {
    let first = match poll_byte(fd, -1)? {
        Some(b) => b,
        None => return Ok(Nav::Ignore),
    };

    match first {
        b'\r' | b'\n' => Ok(Nav::Confirm),
        b' ' => Ok(Nav::Confirm),
        b'j' => Ok(Nav::Down),
        b'k' => Ok(Nav::Up),
        c if (b'1'..=b'3').contains(&c) => Ok(Nav::Number((c - b'1') as usize)),
        b'\x1b' => match poll_byte(fd, 50)? {
            Some(b'[') => match poll_byte(fd, 50)? {
                Some(b'A') => Ok(Nav::Up),
                Some(b'B') => Ok(Nav::Down),
                _ => Ok(Nav::Ignore),
            },
            Some(b'O') => match poll_byte(fd, 50)? {
                Some(b'A') => Ok(Nav::Up),
                Some(b'B') => Ok(Nav::Down),
                _ => Ok(Nav::Ignore),
            },
            _ => Ok(Nav::Ignore),
        },
        _ => Ok(Nav::Ignore),
    }
}

/// Restores the terminal to its original mode on drop.
struct RawModeGuard(libc::termios);
impl Drop for RawModeGuard {
    fn drop(&mut self) {
        unsafe {
            libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &self.0);
        }
    }
}

fn select_language() -> Result<Language> {
    const ITEMS: [&str; 3] = ["Python", "TypeScript / Node.js", "C#"];

    if unsafe { libc::isatty(libc::STDIN_FILENO) } != 1 {
        bail!(
            "Interactive language selection requires a terminal. \
             Re-run with --lang python|typescript|csharp, or run `fusion init` in an interactive terminal."
        );
    }

    // Switch stdin to raw mode so arrow keys arrive immediately and unescaped.
    let fd = libc::STDIN_FILENO;
    let mut original: libc::termios = unsafe { std::mem::zeroed() };
    unsafe {
        libc::tcgetattr(fd, &mut original);
    }
    let mut raw = original;
    unsafe {
        libc::cfmakeraw(&mut raw);
        libc::tcsetattr(fd, libc::TCSAFLUSH, &raw);
    }
    let _raw_guard = RawModeGuard(original);

    let term = Term::stdout();
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
        match read_nav(fd)? {
            Nav::Down => {
                if sel + 1 < ITEMS.len() {
                    sel += 1;
                }
            }
            Nav::Up => {
                if sel > 0 {
                    sel -= 1;
                }
            }
            Nav::Confirm => break,
            Nav::Number(n) => {
                sel = n;
                break;
            }
            Nav::Ignore => {}
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
