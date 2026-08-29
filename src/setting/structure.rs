use crate::setting::config::Language;
use anyhow::{Context, Result};
use console::style;
use std::fs;
use std::path::Path;

const PROJECT_NAME_PLACEHOLDER: &str = "__PROJECT_NAME__";

const PYTHON_MAIN: &str = r#"
"""Entry point: register routes, middleware, and start the server."""

import src.modules.products.products  # registers @route classes

from fusion_framework.app import FusionApp
from fusion_framework.config import get_settings, load_settings_module
from fusion_framework.middleware import (
    cache_headers,
    cors,
    default_builtin_middleware,
    framework_headers,
    request_id,
    security_headers,
)


def _middleware_section(settings, name: str) -> dict:
    block = settings.get("middleware", {}) or {}
    if not isinstance(block, dict):
        block = dict(block) if hasattr(block, "items") else {}
    section = block.get(name, {}) or {}
    return dict(section) if isinstance(section, dict) else {}


def build_middleware(settings) -> list:
    # Method 1: wire each built-in from fusion.<env>.json
    return [
        framework_headers(),
        security_headers(_middleware_section(settings, "security")),
        cors(_middleware_section(settings, "cors")),
        cache_headers(_middleware_section(settings, "cache")),
        request_id(_middleware_section(settings, "request_id")),
    ]
    # Method 2 (alternative): return default_builtin_middleware(settings)


def main() -> None:
    load_settings_module("settings")
    settings = get_settings()
    app = FusionApp(settings)
    for middleware in build_middleware(settings):
        app.use(middleware)
    app.listen()


if __name__ == "__main__":
    main()

"#;

const PYTHON_PRODUCTS: &str = r#"
# Fusion Framework — application route module (FMA)
# Docs:     https://fusion.cipherunit.xyz/
# Desktop:  https://fusion.cipherunit.xyz/en/gui
# CLI tool: https://github.com/cipherunits/fusion-tool

from fusion_framework import status
from fusion_framework.api import FusionBaseApi
from fusion_framework.http_route import http_get
from fusion_framework.route import route


@route(
    "api/[module]/",
    tags=["products"],
    desc="Product resource",
    version="v1",
    deprecated=False,
)
class ProductModule(FusionBaseApi):
    """Product management route module."""

    def get(self):
        # GET /v1/api/product/
        return self.response({"products_id": 12}, status=status.HTTP_SUCCESS)

    def post(self):
        return self.response({"products_id": 12}, status=status.HTTP_201_CREATED)

    def delete(self):
        return self.response({"products_id": 12}, status=status.HTTP_204_NO_CONTENT)

    def patch(self):
        return self.response({"products_id": 12}, status=status.HTTP_SUCCESS)

    @http_get("catalog/[action]", title="Product catalog", tags=["products"])
    def CatalogAction(self):
        # GET /v1/api/product/catalog/catalog
        return self.response({"items": []}, status=status.HTTP_SUCCESS)

"#;

const PYTHON_SETTINGS: &str = r#"
# Fusion Framework settings overlay (UPPERCASE names are merged into settings).
# Runtime values primarily come from fusion.<env>.json (FUSION_ENV, default: dev).
# Docs: https://fusion.cipherunit.xyz/

from fusion_framework import settings

# Never commit real secrets — prefer ALL_CAPS env placeholders in fusion.<env>.json
SECRET_KEY = settings.get("secret_key")

# Prefer reading debug from JSON; keep False as a safe default here
DEBUG = settings.get("debug", default=False)

"#;

const TYPESCRIPT_MAIN: &str = r#"
/**
 * Entry point: register routes, middleware, and start the server.
 */
import "./src/modules/products/products";

import {
  FusionApp,
  cacheHeaders,
  cors,
  frameworkHeaders,
  getSettings,
  requestId,
  securityHeaders,
  settings,
} from "fusion-framework";

function middlewareSection(name: string): Record<string, unknown> {
  const root = settings.get("middleware", {}) as Record<string, unknown> | null;
  const block = root?.[name];
  return block && typeof block === "object" ? (block as Record<string, unknown>) : {};
}

function buildMiddleware() {
  // Method 1: wire each built-in from fusion.<env>.json
  return [
    frameworkHeaders(),
    securityHeaders(middlewareSection("security")),
    cors(middlewareSection("cors")),
    cacheHeaders(middlewareSection("cache")),
    requestId(middlewareSection("request_id")),
  ];
  // Method 2 (alternative): return defaultBuiltinMiddleware()
}

async function main() {
  settings.ensureLoaded([process.cwd()]);
  const app = new FusionApp(getSettings());
  for (const mw of buildMiddleware()) app.use(mw);
  await app.listen();
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
"#;

const TYPESCRIPT_PRODUCTS: &str = r#"
// Fusion Framework — application route module (FMA)
// Docs:     https://fusion.cipherunit.xyz/
// Desktop:  https://fusion.cipherunit.xyz/en/gui
// CLI tool: https://github.com/cipherunits/fusion-tool

import { FusionBaseApi, httpGet, route, status } from "fusion-framework";

class ProductModule extends FusionBaseApi {
  get() {
    // GET /v1/api/product/
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

  CatalogAction() {
    // GET /v1/api/product/catalog/catalog
    return this.response({ items: [] }, status.HTTP_SUCCESS);
  }
}

httpGet("catalog/[action]", {
  title: "Product catalog",
  tags: ["products"],
})(ProductModule.prototype.CatalogAction);

route("api/[module]/", {
  tags: ["products"],
  desc: "Product resource",
  version: "v1",
  deprecated: false,
})(ProductModule);

export { ProductModule };
"#;

const TYPESCRIPT_SETTINGS: &str = r#"
// Fusion Framework settings overlay.
// Values come from fusion.<env>.json (FUSION_ENV, default: dev).

import { settings } from "fusion-framework";

settings.ensureLoaded([process.cwd()]);

export const SECRET_KEY = settings.get("secret_key");
export const DEBUG = settings.get("debug", false);
"#;

const CSHARP_MAIN: &str = r#"
// Fusion Framework entry point
using System.Collections.Generic;
using System.Text.Json.Nodes;
using FusionFramework;

static class Program
{
    static JsonObject MiddlewareSection(FusionSettings settings, string name)
    {
        var root = settings.Get("middleware", new { }) as JsonObject ?? new JsonObject();
        return root[name] as JsonObject ?? new JsonObject();
    }

    static List<FusionMiddleware> BuildMiddleware(FusionSettings settings) => new()
    {
        // Method 1: wire each built-in from fusion.<env>.json
        Middleware.FrameworkHeaders(),
        BuiltinMiddleware.SecurityHeaders(MiddlewareSection(settings, "security")),
        BuiltinMiddleware.Cors(MiddlewareSection(settings, "cors")),
        BuiltinMiddleware.CacheHeaders(MiddlewareSection(settings, "cache")),
        BuiltinMiddleware.RequestId(MiddlewareSection(settings, "request_id")),
    };
    // Method 2 (alternative): BuiltinMiddleware.FromSettings(settings)

    static void Main()
    {
        Route.RegisterAll(typeof(Program).Assembly);
        SettingsStore.Current.EnsureLoaded(System.IO.Directory.GetCurrentDirectory());
        var settings = SettingsStore.GetSettings();
        using var app = new FusionApp(settings);
        foreach (var mw in BuildMiddleware(settings))
            app.Use(mw);
        app.Listen();
    }
}
"#;

const CSHARP_PRODUCTS: &str = r#"
// Fusion Framework — application route module (FMA)
// Docs:     https://fusion.cipherunit.xyz/
// Desktop:  https://fusion.cipherunit.xyz/en/gui
// CLI tool: https://github.com/cipherunits/fusion-tool

using FusionFramework;

namespace Products;

[Route("api/[module]", Tags = new[] { "products" }, Desc = "Product resource", Version = "v1")]
public class ProductModule : FusionBaseApi
{
    // GET /v1/api/product
    public object Get() =>
        Response(new { products_id = 12 }, Status.HTTP_SUCCESS);

    public object Post() =>
        Response(new { products_id = 12 }, Status.HTTP_201_CREATED);

    public object Delete() =>
        Response(new { products_id = 12 }, Status.HTTP_204_NO_CONTENT);

    public object Patch() =>
        Response(new { products_id = 12 }, Status.HTTP_SUCCESS);

    // GET /v1/api/product/catalog/catalog
    [HttpGet("catalog/[action]", Title = "Product catalog")]
    public object CatalogAction() =>
        Response(new { items = Array.Empty<object>() }, Status.HTTP_SUCCESS);
}
"#;

const CSHARP_SETTINGS: &str = r#"
// Fusion Framework settings overlay.
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
        Language::Python => {
            let pyproject = format!(
                r#"[project]
name = "{name}"
version = "0.1.0"
description = "Fusion Framework application"
requires-python = ">=3.9"
"#,
                name = project_name.replace('_', "-"),
            );
            write(target_dir.join("pyproject.toml").as_path(), &pyproject)?;
            Ok(())
        }
        Language::TypeScript => {
            let package_json = format!(
                r#"{{
  "name": "{name}",
  "version": "0.1.0",
  "private": true,
  "type": "module",
  "scripts": {{
    "start": "node main.ts"
  }}
}}
"#,
                name = project_name,
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
    "outDir": "dist",
    "types": ["node"]
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
    <TargetFramework>net10.0</TargetFramework>
    <ImplicitUsings>enable</ImplicitUsings>
    <Nullable>enable</Nullable>
    <RootNamespace>{ns}</RootNamespace>
    <AutomaticallyUseReferenceAssemblyPackages>false</AutomaticallyUseReferenceAssemblyPackages>
  </PropertyGroup>
</Project>
"#,
                ns = project_name.replace('-', "_"),
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

    fn temp_dir(label: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "fusion-structure-test-{}-{}",
            label,
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn test_python_layout_is_created() {
        let target_dir = temp_dir("python");

        create(&target_dir, &Language::Python, "my-app").unwrap();

        assert!(target_dir.join("main.py").is_file());
        assert!(target_dir.join("core/settings.py").is_file());
        assert!(target_dir.join("src/modules/products/products.py").is_file());
        assert!(target_dir.join("pyproject.toml").is_file());

        let main = fs::read_to_string(target_dir.join("main.py")).unwrap();
        assert!(main.contains("registers @route classes"));
        assert!(main.contains("framework_headers"));
        assert!(main.contains("security_headers"));
        assert!(main.contains("build_middleware"));
        assert!(!main.contains("@router"));

        let products = fs::read_to_string(target_dir.join("src/modules/products/products.py")).unwrap();
        assert!(products.contains("http_get"));
        assert!(products.contains("CatalogAction"));
        assert!(products.contains("version=\"v1\""));

        let pyproject = fs::read_to_string(target_dir.join("pyproject.toml")).unwrap();
        assert!(!pyproject.contains("dependencies"));
        assert!(!pyproject.contains("fusion-framework"));

        fs::remove_dir_all(&target_dir).unwrap();
    }

    #[test]
    fn test_typescript_layout_is_created() {
        let target_dir = temp_dir("typescript");

        create(&target_dir, &Language::TypeScript, "my-app").unwrap();

        assert!(target_dir.join("main.ts").is_file());
        assert!(target_dir.join("package.json").is_file());
        assert!(target_dir.join("tsconfig.json").is_file());
        assert!(target_dir.join("src/modules/products/products.ts").is_file());

        let package = fs::read_to_string(target_dir.join("package.json")).unwrap();
        assert!(!package.contains("dependencies"));
        assert!(!package.contains("devDependencies"));
        assert!(!package.contains("fusion-framework"));

        let products = fs::read_to_string(target_dir.join("src/modules/products/products.ts")).unwrap();
        assert!(products.contains("httpGet"));
        assert!(products.contains("CatalogAction"));

        let main = fs::read_to_string(target_dir.join("main.ts")).unwrap();
        assert!(main.contains("frameworkHeaders"));
        assert!(main.contains("securityHeaders"));
        assert!(main.contains("buildMiddleware"));
        assert!(!main.contains("ships with none by default"));

        fs::remove_dir_all(&target_dir).unwrap();
    }

    #[test]
    fn test_csharp_layout_is_created() {
        let target_dir = temp_dir("csharp");

        create(&target_dir, &Language::AspNetCore, "my-app").unwrap();

        assert!(target_dir.join("main.cs").is_file());
        assert!(target_dir.join("my-app.csproj").is_file());
        assert!(target_dir.join("src/modules/products/products.cs").is_file());

        let csproj = fs::read_to_string(target_dir.join("my-app.csproj")).unwrap();
        assert!(csproj.contains("net10.0"));
        assert!(!csproj.contains("PackageReference"));
        assert!(!csproj.contains("FusionFramework"));

        let products = fs::read_to_string(target_dir.join("src/modules/products/products.cs")).unwrap();
        assert!(products.contains("[HttpGet"));
        assert!(products.contains("CatalogAction"));

        let main = fs::read_to_string(target_dir.join("main.cs")).unwrap();
        assert!(main.contains("FrameworkHeaders"));
        assert!(main.contains("SecurityHeaders"));
        assert!(main.contains("BuildMiddleware"));
        assert!(!main.contains("ships with none by default"));

        fs::remove_dir_all(&target_dir).unwrap();
    }
}
