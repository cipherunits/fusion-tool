use crate::setting::config::Language;
use anyhow::{Context, Result};
use console::style;
use std::fs;
use std::path::Path;

const PROJECT_NAME_PLACEHOLDER: &str = "__PROJECT_NAME__";

const PYTHON_MAIN: &str = r#"from core.settings import CONFIG, ENV, PROJECT_NAME


def main() -> None:
    print(f"{PROJECT_NAME} is running in {ENV} mode on port {CONFIG.get('port')}")


if __name__ == "__main__":
    main()
"#;

const PYTHON_SETTINGS: &str = r#"import json
import os
from pathlib import Path

BASE_DIR = Path(__file__).resolve().parent.parent

PROJECT_NAME = "__PROJECT_NAME__"

ENV = os.getenv("FUSION_ENV", "dev")


def load_config(env: str = ENV) -> dict:
    """Read the config block of fusion<env>.json in the project root."""
    config_file = BASE_DIR / f"fusion{env}.json"

    if not config_file.exists():
        return {}

    with config_file.open(encoding="utf-8") as file:
        return json.load(file).get("config", {})


CONFIG = load_config()
"#;

const TYPESCRIPT_MAIN: &str = r#"import { ENV, PROJECT_NAME } from "./core/settings";

function main(): void {
  console.log(`${PROJECT_NAME} is running in ${ENV} mode`);
}

main();
"#;

const TYPESCRIPT_SETTINGS: &str = r#"export const PROJECT_NAME = "__PROJECT_NAME__";

export const ENV = process.env.FUSION_ENV ?? "dev";
"#;

const CSHARP_MAIN: &str = r#"public static class Program
{
    public static void Main()
    {
        System.Console.WriteLine(
            $"{Settings.ProjectName} is running in {Settings.Env} mode"
        );
    }
}
"#;

const CSHARP_SETTINGS: &str = r#"public static class Settings
{
    public const string ProjectName = "__PROJECT_NAME__";

    public static string Env =>
        System.Environment.GetEnvironmentVariable("FUSION_ENV") ?? "dev";
}
"#;

/// Directories every new project starts with. `src/modules` also creates `src`.
const DIRECTORIES: [&str; 2] = ["core", "src/modules"];

/// Create the starting layout of a new project:
///
/// ```text
/// ├── core
/// │   └── settings.py
/// ├── main.py
/// └── src
///     └── modules
/// ```
pub fn create(target_dir: &Path, language: &Language, project_name: &str) -> Result<()> {
    for directory in DIRECTORIES {
        let path = target_dir.join(directory);

        fs::create_dir_all(&path)
            .with_context(|| format!("Could not create {}", path.display()))?;

        report(&path);
    }

    let (main_template, settings_template) = templates(language);

    let extension = language.extension();

    write(
        &target_dir.join(format!("main{}", extension)),
        &render(main_template, project_name),
    )?;

    write(
        &target_dir
            .join("core")
            .join(format!("settings{}", extension)),
        &render(settings_template, project_name),
    )?;

    Ok(())
}

/// Entry point and settings templates for a language
fn templates(language: &Language) -> (&'static str, &'static str) {
    match language {
        Language::Python => (PYTHON_MAIN, PYTHON_SETTINGS),

        Language::TypeScript => (TYPESCRIPT_MAIN, TYPESCRIPT_SETTINGS),

        Language::AspNetCore => (CSHARP_MAIN, CSHARP_SETTINGS),
    }
}

fn render(template: &str, project_name: &str) -> String {
    template.replace(PROJECT_NAME_PLACEHOLDER, project_name)
}

fn write(path: &Path, content: &str) -> Result<()> {
    fs::write(path, content).with_context(|| format!("Could not create {}", path.display()))?;

    report(path);

    Ok(())
}

fn report(path: &Path) {
    println!(
        "{}",
        style(format!("✔ {} created successfully!", path.display()))
            .green()
            .bold()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_layout_is_created() {
        let target_dir =
            std::env::temp_dir().join(format!("fusion-structure-test-{}", std::process::id()));

        fs::create_dir_all(&target_dir).unwrap();

        create(&target_dir, &Language::Python, "my-app").unwrap();

        assert!(target_dir.join("main.py").is_file());
        assert!(target_dir.join("core/settings.py").is_file());
        assert!(target_dir.join("src/modules").is_dir());

        let settings = fs::read_to_string(target_dir.join("core/settings.py")).unwrap();

        assert!(settings.contains("PROJECT_NAME = \"my-app\""));
        assert!(!settings.contains(PROJECT_NAME_PLACEHOLDER));

        fs::remove_dir_all(&target_dir).unwrap();
    }
}
