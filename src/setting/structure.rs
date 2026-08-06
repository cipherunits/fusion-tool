use crate::setting::config::Language;
use anyhow::{Context, Result};
use console::style;
use std::fs;
use std::path::Path;

const PROJECT_NAME_PLACEHOLDER: &str = "__PROJECT_NAME__";

const PYTHON_MAIN: &str = r#"
"""Entry point: import API classes, then start from settings."""

import src.modules.products.products #registers @router classes
from fusion_framework.app import run

if __name__ == "__main__":
    run("settings")

"#;

const PYTHON_PRODUCTS: &str = r#"
from fusion_frameworka.api import FusionBaseApi
from fusion_framework.route import router

@router("api/[module]/")
class ProductModule:
    """Product management module."""

    def get(self):
        return {"products_id": 12} 
"#;

const PYTHON_SETTINGS: &str = r#"

# Fusion Framework Settings
# --------------------------------------------
# This file contains the core configuration
# for your application.
#
# License: MIT License
# You are free to use, modify, and distribute.


# variables or external config providers.
from fusion_framework import settings


# Never expose your secret key in public repositories
SECRET_KEY = settings.get("secret_key")

# Enable debug mode (DO NOT use True in production)
DEBUG = settings.get("debug", defualt=False)

"#;

const TYPESCRIPT_MAIN: &str = r#"import { ENV, PROJECT_NAME } from "./core/settings";

function main(): void {
  console.log(`${PROJECT_NAME} is running in ${ENV} mode`);
}

main();
"#;

const TYPESCRIPT_PRODUCTS: &str = r#"import { module } from "fusion-framework";

@module
class ProductModule {
  name = "products";

  register(): void {
  }
}"#;

const TYPESCRIPT_SETTINGS: &str = r#"

// ============================================
// Fusion Framework Settings
// --------------------------------------------
// This file contains the core configuration
// for your application.
//
// License: MIT License
// You are free to use, modify, and distribute.
// ============================================

// variables or external config providers.
const settings = require("fusion-framework");

// Never expose your secret key in public repositories
const SECRET_KEY = settings.get("secret_key");

// Enable debug mode (DO NOT use true in production)
const DEBUG = settings.get("debug", { default: false });

module.exports = {
  SECRET_KEY,
  DEBUG,
};
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

const CSHARP_PRODUCTS: &str = r#"public static class ProductModule
{
    public string Name => "products";

    public void Register()
    {
    }
}"#;

const CSHARP_SETTINGS: &str = r#"public static class Settings
{
    public const string ProjectName = "__PROJECT_NAME__";

    public static string Env =>
        System.Environment.GetEnvironmentVariable("FUSION_ENV") ?? "dev";
}
"#;

/// Directories every new project starts with. `src/modules` also creates `src`.
const DIRECTORIES: [&str; 3] = ["core", "src/modules", "src/modules/products"];

/// Create the starting layout of a new project:
///
/// ```text
/// ├── core
/// │   └── settings.py
/// ├── main.py
/// └── src
///     └── modules
///         └── products
///             └── products.py
/// ```
pub fn create(target_dir: &Path, language: &Language, project_name: &str) -> Result<()> {
    for directory in DIRECTORIES {
        let path = target_dir.join(directory);

        fs::create_dir_all(&path)
            .with_context(|| format!("Could not create {}", path.display()))?;

        report(&path);
    }

    let (main_template, settings_template, _) = templates(language);

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

    let products_template = products_template(language);

    write(
        &target_dir
            .join("src/modules/products")
            .join(format!("products{}", extension)),
        &render(products_template, project_name),
    )?;

    Ok(())
}

/// Entry point, settings, and products templates for a language
fn templates(language: &Language) -> (&'static str, &'static str, &'static str) {
    match language {
        Language::Python => (PYTHON_MAIN, PYTHON_SETTINGS, PYTHON_PRODUCTS),

        Language::TypeScript => (TYPESCRIPT_MAIN, TYPESCRIPT_SETTINGS, TYPESCRIPT_PRODUCTS),

        Language::AspNetCore => (CSHARP_MAIN, CSHARP_SETTINGS, CSHARP_PRODUCTS),
    }
}

fn products_template(language: &Language) -> &'static str {
    match language {
        Language::Python => PYTHON_PRODUCTS,
        Language::TypeScript => TYPESCRIPT_PRODUCTS,
        Language::AspNetCore => CSHARP_PRODUCTS,
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
        assert!(target_dir.join("src/modules/products").is_dir());
        assert!(target_dir.join("src/modules/products/products.py").is_file());

        let settings = fs::read_to_string(target_dir.join("core/settings.py")).unwrap();

        assert!(settings.contains("SECRET_KEY = settings.get(\"secret_key\")"));
        assert!(!settings.contains(PROJECT_NAME_PLACEHOLDER));

        fs::remove_dir_all(&target_dir).unwrap();
    }
}
