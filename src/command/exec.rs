use crate::setting::{environment, environment::Environment, get};
use anyhow::{bail, Context, Result};
use console::style;
use std::process;

/// Environment used when none is requested and FUSION_ENV is unset
const DEFAULT_ENV: &str = "dev";

#[cfg(windows)]
const SHELL: (&str, &str) = ("cmd", "/C");

#[cfg(not(windows))]
const SHELL: (&str, &str) = ("sh", "-c");

/// Run a command declared in the `commands` block of an environment file.
///
/// The environment can come from the command itself (`run:stage`) or from a
/// flag (`--stage`, `--env stage`).
pub fn exec(name: Option<String>, env: Option<String>) -> Result<()> {
    let project_root = get::get_project_root();

    let (name, env) = resolve(name, env)?;

    let environment = environment::read(&project_root, &env)?;

    let Some(name) = name else {
        return list(&environment, &env);
    };

    let command = environment.commands.get(&name).ok_or_else(|| {
        anyhow::anyhow!(
            "'{}' is not declared in {}.{}",
            name,
            environment::file_name(&env),
            available(&environment)
        )
    })?;

    println!();
    println!(
        "{} {}",
        style(format!("[{}]", env)).cyan().bold(),
        style(command).bold()
    );
    println!();

    let (shell, argument) = SHELL;

    let status = process::Command::new(shell)
        .arg(argument)
        .arg(command)
        .current_dir(&project_root)
        // Lets the project read the matching config, the same variable
        // core/settings reads.
        .env("FUSION_ENV", &env)
        .status()
        .with_context(|| format!("Could not run '{}'", command))?;

    if !status.success() {
        process::exit(status.code().unwrap_or(1));
    }

    Ok(())
}

/// Split `run:stage` into its command and environment, then settle on the
/// environment to use.
fn resolve(name: Option<String>, env: Option<String>) -> Result<(Option<String>, String)> {
    let (name, inline_env) = match name {
        Some(name) => match name.split_once(':') {
            Some((name, env)) if !env.is_empty() => (Some(name.to_string()), Some(env.to_string())),

            _ => (Some(name), None),
        },

        None => (None, None),
    };

    if let (Some(inline), Some(flag)) = (&inline_env, &env) {
        if inline != flag {
            bail!(
                "Two environments requested: '{}' and '{}'. Pick one.",
                inline,
                flag
            );
        }
    }

    let env = inline_env
        .or(env)
        .or_else(|| std::env::var("FUSION_ENV").ok())
        .unwrap_or_else(|| DEFAULT_ENV.to_string());

    Ok((name, env))
}

/// Show what the environment declares, since commands are project specific
fn list(environment: &Environment, env: &str) -> Result<()> {
    let file = environment::file_name(env);

    if environment.commands.is_empty() {
        bail!(
            "No commands are declared in {}. Add them under \"commands\".",
            file
        );
    }

    let width = environment
        .commands
        .keys()
        .map(String::len)
        .max()
        .unwrap_or(0);

    println!();
    println!("{}", style(format!("Commands in {}", file)).cyan().bold());
    println!();

    for (name, command) in &environment.commands {
        println!("  {:<width$}  {}", style(name).yellow(), command);
    }

    println!();

    Ok(())
}

fn available(environment: &Environment) -> String {
    if environment.commands.is_empty() {
        return " No commands are declared yet.".to_string();
    }

    format!(
        " Available: {}",
        environment
            .commands
            .keys()
            .cloned()
            .collect::<Vec<_>>()
            .join(", ")
    )
}

/// Collapse the environment flags into a single choice
pub fn selected_env(env: Option<String>, dev: bool, stage: bool, prod: bool) -> Option<String> {
    if let Some(env) = env {
        return Some(env);
    }

    if dev {
        return Some("dev".to_string());
    }

    if stage {
        return Some("stage".to_string());
    }

    if prod {
        return Some("prod".to_string());
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn resolved(name: &str, env: Option<&str>) -> (Option<String>, String) {
        resolve(Some(name.to_string()), env.map(String::from)).unwrap()
    }

    #[test]
    fn test_inline_environment() {
        assert_eq!(
            resolved("run:stage", None),
            (Some("run".to_string()), "stage".to_string())
        );
    }

    #[test]
    fn test_flag_environment() {
        assert_eq!(
            resolved("run", Some("prod")),
            (Some("run".to_string()), "prod".to_string())
        );
    }

    #[test]
    fn test_defaults_to_dev() {
        // FUSION_ENV is read only when nothing else is given, and the test
        // environment does not set it.
        assert_eq!(
            resolved("run", None),
            (Some("run".to_string()), DEFAULT_ENV.to_string())
        );
    }

    #[test]
    fn test_trailing_colon_is_part_of_no_environment() {
        assert_eq!(
            resolved("run:", None),
            (Some("run:".to_string()), DEFAULT_ENV.to_string())
        );
    }

    #[test]
    fn test_conflicting_environments() {
        assert!(resolve(Some("run:stage".to_string()), Some("prod".to_string())).is_err());

        assert!(resolve(Some("run:stage".to_string()), Some("stage".to_string())).is_ok());
    }

    #[test]
    fn test_selected_env() {
        assert_eq!(
            selected_env(None, false, true, false).as_deref(),
            Some("stage")
        );
        assert_eq!(
            selected_env(Some("test".into()), false, false, false).as_deref(),
            Some("test")
        );
        assert_eq!(selected_env(None, false, false, false), None);
    }
}
