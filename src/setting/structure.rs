use crate::setting::config::Language;
use anyhow::{Context, Result};
use console::style;
use std::fs;
use std::path::Path;

const PROJECT_NAME_PLACEHOLDER: &str = "__PROJECT_NAME__";

const PYTHON_MAIN: &str = r#"
"""Entry point: register routes, middleware, and start the server."""

import src.modules.products.products  # registers @router classes

from fusion_framework.app import FusionApp
from fusion_framework.config import get_settings, load_settings_module

# Global middleware (optional).
# Each item: (request, call_next) -> response | call_next(request)
# Example:
#   from fusion_framework import bearer_jwt
#   MIDDLEWARE = [bearer_jwt()]
MIDDLEWARE: list = []


def main() -> None:
    load_settings_module("settings")
    app = FusionApp(get_settings())
    for middleware in MIDDLEWARE:
        app.use(middleware)
    app.listen()


if __name__ == "__main__":
    main()

"#;

const PYTHON_PRODUCTS: &str = r#"
# Fusion Framework
# Docs:     https://fusion.cipherunit.xyz/
# Desktop:  https://fusion.cipherunit.xyz/en/gui
# CLI tool: https://github.com/cipherunits/fusion-tool


from fusion_framework.api import FusionBaseApi
from fusion_framework.route import route
from fusion_framework import status

@route(
      "api/[module]/",
      tags=["swagger"],
      desc="Fusion Framework Api",
      version="v1",
      deprecated=False
      )
class ProductModule(FusionBaseApi):
    """Product management module."""

    def get(self):
        return self.response({"products_id": 12},status=status.HTTP_SUCCESS)

    def post(self):
            return self.response({"products_id": 12},status=status.HTTP_201_CREATED)
    
    def delete(self):
            return self.response({"products_id": 12},status=status.HTTP_204_NO_CONTENT)
    
    def patch(self):
            return self.response({"products_id": 12},status=status.HTTP_SUCCESS)
        
"#;

const PYTHON_SETTINGS: &str = r#"
# Docs: https://fusion.cipherunit.xyz/

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
DEBUG = settings.get("debug", default=False)

"#;

const TYPESCRIPT_MAIN: &str = r#"
/**
 * Entry point: register routes, middleware, and start the server.
 */
import "./src/modules/products/products";

import { FusionApp, getSettings, settings } from "fusion-framework";

// Global middleware (optional). Framework ships with none by default.
// Example: import { bearerJwt } from "fusion-framework"; const MIDDLEWARE = [bearerJwt()];
const MIDDLEWARE: Array<(req: any, next: any) => any> = [];

async function main() {
  settings.ensureLoaded([process.cwd()]);
  const app = new FusionApp(getSettings());
  for (const mw of MIDDLEWARE) app.use(mw);
  await app.listen();
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
"#;

const TYPESCRIPT_PRODUCTS: &str = r#"
// Fusion Framework
// Docs:     https://fusion.cipherunit.xyz/
// Desktop:  https://fusion.cipherunit.xyz/en/gui
// CLI tool: https://github.com/cipherunits/fusion-tool

import { FusionBaseApi, route, status } from "fusion-framework";

export const ProductModule = route("api/[module]/", {
  tags: ["swagger"],
  desc: "Fusion Framework Api",
  version: "v1",
  deprecated: false,
})(
  class ProductModule extends FusionBaseApi {
    get() {
      return this.response({ products_id: 12 }, status.HTTP_SUCCESS);
    }

    post() {
      return this.response({ products_id: 12 }, status.HTTP_201_CREATED);
    }

    delete() {
      return this.response({ products_id: 12 }, status.HTTP_204_NO_CONTENT);
    }

    patch() {
      return this.response({ products_id: 12 }, status.HTTP_SUCCESS);
    }
  },
);
"#;

const TYPESCRIPT_SETTINGS: &str = r#"
// Fusion Framework Settings
// Values come from fusion.<env>.json (FUSION_ENV, default: dev).

import { settings } from "fusion-framework";

settings.ensureLoaded();

export const SECRET_KEY = settings.get("secret_key");
export const DEBUG = settings.get("debug", false);
"#;

const CSHARP_MAIN: &str = r#"
// Fusion Framework entry point
using System.Collections.Generic;
using FusionFramework;

// Global middleware (optional). Framework ships with none by default.
// Example: MIDDLEWARE.Add(Middleware.BearerJwt());
static class Program
{
    static readonly List<FusionMiddleware> MIDDLEWARE = new();

    static void Main()
    {
        Route.RegisterAll(typeof(Program).Assembly);
        SettingsStore.Current.EnsureLoaded(System.IO.Directory.GetCurrentDirectory());
        using var app = new FusionApp(SettingsStore.GetSettings());
        foreach (var mw in MIDDLEWARE)
            app.Use(mw);
        app.Listen();
    }
}
"#;

const CSHARP_PRODUCTS: &str = r#"
// Fusion Framework
// Docs:     https://fusion.cipherunit.xyz/
// Desktop:  https://fusion.cipherunit.xyz/en/gui
// CLI tool: https://github.com/cipherunits/fusion-tool

using FusionFramework;

namespace Products;

[Route("api/[module]", Tags = new[] { "swagger" }, Desc = "Fusion Framework Api", Version = "v1")]
public class ProductModule : FusionBaseApi
{
    public object Get() =>
        Response(new { products_id = 12 }, Status.HTTP_SUCCESS);

    public object Post() =>
        Response(new { products_id = 12 }, Status.HTTP_201_CREATED);

    public object Delete() =>
        Response(new { products_id = 12 }, Status.HTTP_204_NO_CONTENT);

    public object Patch() =>
        Response(new { products_id = 12 }, Status.HTTP_SUCCESS);
}
"#;

const CSHARP_SETTINGS: &str = r#"
// Fusion Framework Settings
// Values come from fusion.<env>.json (FUSION_ENV, default: dev).

using FusionFramework;

public static class CoreSettings
{
    static CoreSettings()
    {
        SettingsStore.Current.EnsureLoaded();
    }

    public static object? SecretKey => SettingsStore.Current.Get("secret_key");
    public static object? Debug => SettingsStore.Current.Get("debug", false);
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

    write_language_project_files(target_dir, language, project_name)?;

    Ok(())
}

fn write_language_project_files(
    target_dir: &Path,
    language: &Language,
    project_name: &str,
) -> Result<()> {
    match language {
        Language::Python => Ok(()),
        Language::TypeScript => {
            let package_json = format!(
                r#"{{
  "name": "{name}",
  "version": "0.1.0",
  "private": true,
  "type": "module",
  "scripts": {{
    "dev": "tsx main.ts",
    "start": "tsx main.ts"
  }},
  "dependencies": {{
    "fusion-framework": "{version}"
  }},
  "devDependencies": {{
    "tsx": "^4.19.0",
    "typescript": "^5.6.0"
  }}
}}
"#,
                name = project_name,
                version = crate::setting::FUSION_FRAMEWORK_VERSION,
            );
            write(&target_dir.join("package.json"), &package_json)?;
            write(
                &target_dir.join("tsconfig.json"),
                r#"{
  "compilerOptions": {
    "target": "ES2022",
    "module": "NodeNext",
    "moduleResolution": "NodeNext",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "outDir": "dist"
  },
  "include": ["**/*.ts"]
}
"#,
            )?;
            Ok(())
        }
        Language::AspNetCore => {
            let csproj = format!(
                r#"<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net8.0</TargetFramework>
    <ImplicitUsings>enable</ImplicitUsings>
    <Nullable>enable</Nullable>
    <RootNamespace>{ns}</RootNamespace>
    <AutomaticallyUseReferenceAssemblyPackages>false</AutomaticallyUseReferenceAssemblyPackages>
  </PropertyGroup>
  <ItemGroup>
    <!-- Prefer the published NuGet package when available. -->
    <PackageReference Include="FusionFramework" Version="{version}" />
  </ItemGroup>
</Project>
"#,
                ns = project_name.replace('-', "_"),
                version = crate::setting::FUSION_FRAMEWORK_VERSION,
            );
            write(
                &target_dir.join(format!("{project_name}.csproj")),
                &csproj,
            )?;
            Ok(())
        }
    }
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
