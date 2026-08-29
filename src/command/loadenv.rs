use crate::command::selected_env;
use crate::setting::{environment, get};
use anyhow::{Context, Result};
use console::style;
use std::collections::BTreeMap;
use std::process;

/// Environment used when none is requested and FUSION_ENV is unset
const DEFAULT_ENV: &str = "dev";

/// Load `fusion.<env>.json` into process environment variables.
///
/// Without a trailing command, prints shell exports for:
/// `eval "$(fusion loadenv --stage)"`
///
/// With a command, runs it with the loaded variables in the child process:
/// `fusion loadenv --stage -- python main.py`
pub fn loadenv(env: Option<String>, dev: bool, stage: bool, prod: bool, command: Vec<String>) -> Result<()> {
    let project_root = get::get_project_root();
    let env = selected_env(env, dev, stage, prod)
        .or_else(|| std::env::var("FUSION_ENV").ok())
        .unwrap_or_else(|| DEFAULT_ENV.to_string());

    let environment = environment::read(&project_root, &env)?;
    let mut vars = environment::settings_to_env_vars(&environment.settings());
    vars.insert("FUSION_ENV".to_string(), env.clone());

    if command.is_empty() {
        print_exports(&env, &vars);
        return Ok(());
    }

    run_with_env(&project_root, &vars, &command)
}

fn print_exports(env: &str, vars: &BTreeMap<String, String>) {
    println!(
        "# fusion loadenv ({env}) — eval: eval \"$(fusion loadenv --{env})\""
    );

    for (key, value) in vars {
        println!("export {}={}", key, shell_escape(value));
    }
}

fn shell_escape(value: &str) -> String {
    if value
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-' | '/' | ':' | '@'))
    {
        value.to_string()
    } else {
        format!("'{}'", value.replace('\'', "'\\''"))
    }
}

fn run_with_env(project_root: &std::path::Path, vars: &BTreeMap<String, String>, command: &[String]) -> Result<()> {
    let (program, args) = command
        .split_first()
        .context("No command provided after `--`")?;

    println!();
    println!(
        "{} {}",
        style(format!("[{}]", vars.get("FUSION_ENV").map(String::as_str).unwrap_or("dev"))).cyan().bold(),
        style(command.join(" ")).bold()
    );
    println!();

    let mut child = process::Command::new(program);
    child.args(args).current_dir(project_root);

    for (key, value) in vars {
        child.env(key, value);
    }

    let status = child
        .status()
        .with_context(|| format!("Could not run '{}'", command.join(" ")))?;

    if !status.success() {
        process::exit(status.code().unwrap_or(1));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_escape() {
        assert_eq!(shell_escape("8080"), "8080");
        assert_eq!(shell_escape("hello world"), "'hello world'");
        assert_eq!(shell_escape("it's"), "'it'\\''s'");
    }
}
