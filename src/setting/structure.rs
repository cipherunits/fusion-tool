use crate::setting::config::Language;
use anyhow::{Context, Result};
use console::style;
use std::fs;
use std::path::Path;

const PROJECT_NAME_PLACEHOLDER: &str = "__PROJECT_NAME__";

const PYTHON_MAIN: &str = r#"

"""Entry point: register routes, middleware, and start the server."""

from pathlib import Path

import src.modules.products.products  # registers @route classes

from fusion_framework.app import FusionApp
from fusion_framework.config import get_settings, load_settings_module
from fusion_framework.middleware import (
    cache_headers,
    cors,
    framework_headers,
    request_id,
    security_headers,
    static_files,
)


def main() -> None:
    load_settings_module("settings")
    from core.settings import TEMPLATES_DIR

    app = FusionApp(get_settings())
    app.use(static_files(root=Path(TEMPLATES_DIR) / "home", prefix="/"))  # Delete this if you are using API
    app.use(request_id())
    app.use(cors())
    app.use(cache_headers())
    app.use(security_headers())
    app.use(framework_headers())  # Delete this middleware if you are in production
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
from fusion_framework.template import FusionBaseTemplate


@route("/")
class HomePage(FusionBaseTemplate):
    """Root page rendered (templates/home/index.html)."""

    template = "home/index.html"

    def context(self):
        return {
            "title": "__PROJECT_NAME__",
            "message": (
                "Your Fusion Framework application is ready. "
                "Start building by creating routes, templates, and business logic."
            ),
            "project": "__PROJECT_NAME__",
            "table_headers": ["Route", "Method"],
            "table_rows": [
                ["/v1/api/product", "GET"],
                ["/swagger", "GET"],
            ],
        }


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

# Auto-restart on source changes (True/False — language-native, not JSON)
RELOAD = settings.get("reload", default=True)

# templates root (relative to project cwd)
TEMPLATES_DIR = settings.get("templates.dir", default="templates")

"#;

const TYPESCRIPT_MAIN: &str = r#"
/**
 * Entry point: register routes, middleware, and start the server.
 */
import path from "node:path";

import "./src/modules/products/products";

import {
  FusionApp,
  cacheHeaders,
  cors,
  frameworkHeaders,
  getSettings,
  requestId,
  securityHeaders,
  staticFiles,
} from "fusion-framework";

async function main() {
  const { TEMPLATES_DIR } = await import("./core/settings");

  const app = new FusionApp(getSettings());
  app.use(
    staticFiles({ root: path.join(String(TEMPLATES_DIR), "home"), prefix: "/" }),
  ); // Delete this if you are using API
  app.use(requestId());
  app.use(cors());
  app.use(cacheHeaders());
  app.use(securityHeaders());
  app.use(frameworkHeaders()); // Delete this middleware if you are in production
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

import {
  FusionBaseApi,
  FusionBaseTemplate,
  httpGet,
  route,
  status,
} from "fusion-framework";

class HomePage extends FusionBaseTemplate {
  static template = "home/index.html";

  context() {
    return {
      title: "__PROJECT_NAME__",
      message:
        "Your Fusion Framework application is ready. Start building by creating routes, templates, and business logic.",
      project: "__PROJECT_NAME__",
      table_headers: ["Route", "Method"],
      table_rows: [
        ["/v1/api/product", "GET"],
        ["/swagger", "GET"],
      ],
    };
  }
}

route("/")(HomePage);

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

export { HomePage, ProductModule };
"#;

const TYPESCRIPT_SETTINGS: &str = r#"
// Fusion Framework settings overlay.
// Values come from fusion.<env>.json (FUSION_ENV, default: dev).

import { settings } from "fusion-framework";

settings.ensureLoaded([process.cwd()]);

export const SECRET_KEY = settings.get("secret_key");
export const DEBUG = settings.get("debug", false);
// Auto-restart on source changes (boolean — language-native)
export const RELOAD = settings.get("reload", true) ?? true;
// templates root
export const TEMPLATES_DIR = settings.get("templates.dir", "templates") ?? "templates";

settings.merge({
  reload: RELOAD,
  templates: { dir: TEMPLATES_DIR },
});
"#;

const CSHARP_MAIN: &str = r#"
// Fusion Framework entry point
using System.IO;
using FusionFramework;

static class Program
{
    static void Main()
    {
        Route.RegisterAll(typeof(Program).Assembly);

        SettingsStore.Current.EnsureLoaded(
            Directory.GetCurrentDirectory()
        );
        _ = CoreSettings.Reload;
        var templatesDir = CoreSettings.TemplatesDir?.ToString() ?? "templates";

        var app = new FusionApp(SettingsStore.GetSettings());
        app.Use(BuiltinMiddleware.StaticFiles(
            root: Path.Combine(templatesDir, "home"),
            prefix: "/")); // Delete this if you are using API
        app.Use(BuiltinMiddleware.RequestId());
        app.Use(BuiltinMiddleware.Cors());
        app.Use(BuiltinMiddleware.CacheHeaders());
        app.Use(BuiltinMiddleware.SecurityHeaders());
        app.Use(BuiltinMiddleware.FrameworkHeaders()); // Delete this middleware if you are in production
        app.Listen();
    }
}
"#;

const CSHARP_PRODUCTS: &str = r#"
// Fusion Framework — application route module (FMA)
// Docs:     https://fusion.cipherunit.xyz/
// Desktop:  https://fusion.cipherunit.xyz/en/gui
// CLI tool: https://github.com/cipherunits/fusion-tool

using System.Text.Json.Nodes;
using FusionFramework;

namespace Products;

[Route("/")]
public class HomePage : FusionBaseTemplate
{
    static HomePage()
    {
        Template = "home/index.html";
    }

    public override Dictionary<string, JsonNode?> Context() => new()
    {
        ["title"] = JsonValue.Create("__PROJECT_NAME__"),
        ["message"] = JsonValue.Create(
            "Your Fusion Framework application is ready. Start building by creating routes, templates, and business logic."),
        ["project"] = JsonValue.Create("__PROJECT_NAME__"),
        ["table_headers"] = new JsonArray("Route", "Method"),
        ["table_rows"] = new JsonArray(
            new JsonArray("/v1/api/product", "GET"),
            new JsonArray("/swagger", "GET")),
    };
}

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
    // Auto-restart on source changes
    public static object? Reload => SettingsStore.Current.Get("reload", true);
    // templates root
    public static object? TemplatesDir => SettingsStore.Current.Get("templates.dir", "templates");
}
"#;

const HOME_INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>{{ title }}</title>
  <style>
    {% include "fusion/components.css" %}
    {% include "home/style.css" %}
  </style>
</head>
<body>
  <main class="container">
    <div class="logo">
      <img src="/Fusion-Framework-Transparent.png" alt="Fusion Framework" width="75" />
    </div>

    {{<fusion.badge label="Installation successful" variant="success" dot={true} />}}

    <h1>
      Welcome to <span>Fusion</span>
    </h1>

    <p class="description">
      {{ message }}
    </p>

    {% <fusion.card title="Get started"> %}
      <div class="code">
        <span class="command">$</span> fusion command run
      </div>
    {% </fusion.card> %}

    <div class="links">
      {{<fusion.button label="Documentation" href="/swagger" variant="primary" />}}
      {{<fusion.button label="GitHub" href="https://github.com/cipherunits/fusion-framework" variant="secondary" />}}
    </div>

    {{<fusion.table headers={table_headers} rows={table_rows} caption="Sample API routes" />}}

    <div class="footer">
      Project <strong>{{ project }}</strong> · powered by Fusion Framework
    </div>
  </main>
</body>
</html>
"#;

const HOME_STYLE_CSS: &str = r#"
* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

body {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  font-family: Inter, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
  background: #f8fafc;
  color: #0f172a;
}

.container {
  width: 100%;
  max-width: 760px;
  padding: 40px 24px;
  text-align: center;
}

.logo {
  width: 72px;
  height: 72px;
  margin: 0 auto 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 18px;
  background: #0f172a;
  color: white;
  box-shadow: 0 10px 30px rgba(15, 23, 42, 0.15);
}

.logo img {
  display: block;
  max-width: 48px;
  height: auto;
}

.fusion-badge {
  margin-bottom: 20px;
}

h1 {
  font-size: clamp(34px, 6vw, 52px);
  line-height: 1.1;
  letter-spacing: -1.5px;
  margin-bottom: 18px;
}

h1 span {
  color: #6366f1;
}

.description {
  max-width: 580px;
  margin: 0 auto;
  color: #64748b;
  font-size: 17px;
  line-height: 1.7;
}

.fusion-card {
  margin-top: 42px;
}

.code {
  padding: 15px 17px;
  background: #0f172a;
  color: #e2e8f0;
  border-radius: 9px;
  font-family: "SFMono-Regular", Consolas, "Liberation Mono", monospace;
  font-size: 14px;
  line-height: 1.6;
  overflow-x: auto;
}

.code .command {
  color: #a5b4fc;
}

.links {
  margin-top: 28px;
  display: flex;
  justify-content: center;
  gap: 12px;
  flex-wrap: wrap;
}

.fusion-table-wrap {
  margin-top: 28px;
  text-align: left;
}

.footer {
  margin-top: 42px;
  color: #94a3b8;
  font-size: 13px;
}

@media (max-width: 600px) {
  .container {
    padding: 30px 18px;
  }
  .fusion-card {
    margin-top: 32px;
  }
  .description {
    font-size: 15px;
  }
  .links {
    flex-direction: column;
  }
  .fusion-btn {
    width: 100%;
  }
}
"#;

/// Directories every new project starts with. `src/modules` also creates `src`.
const DIRECTORIES: [&str; 4] = [
    "core",
    "src/modules",
    "src/modules/products",
    "templates/home",
];

/// Default home page logo copied into `templates/home/` by `fusion init`.
const HOME_LOGO_PNG: &[u8] =
    include_bytes!("../../assets/Fusion-Framework-Transparent.png");

/// Create the starting layout of a new project:
///
/// ```text
/// ├── core
/// │   └── settings.py
/// ├── main.py
/// ├── templates
/// │   └── home
/// │       ├── index.html
/// │       ├── style.css
/// │       └── Fusion-Framework-Transparent.png
/// └── src
///     └── modules
///         └── products
///             └── products.py   # API + HomePage (FusionBaseTemplate)
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

    write(
        &target_dir.join("templates/home/index.html"),
        &render(HOME_INDEX_HTML, project_name),
    )?;
    write(
        &target_dir.join("templates/home/style.css"),
        HOME_STYLE_CSS,
    )?;

    let logo_path = target_dir.join("templates/home/Fusion-Framework-Transparent.png");
    fs::write(&logo_path, HOME_LOGO_PNG)
        .with_context(|| format!("Could not create {}", logo_path.display()))?;
    report(&logo_path);

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
        assert!(target_dir.join("templates/home/index.html").is_file());
        assert!(target_dir.join("templates/home/style.css").is_file());
        assert!(target_dir
            .join("templates/home/Fusion-Framework-Transparent.png")
            .is_file());
        assert!(target_dir.join("pyproject.toml").is_file());

        let main = fs::read_to_string(target_dir.join("main.py")).unwrap();
        assert!(main.contains("registers @route classes"));
        assert!(main.contains("static_files"));
        assert!(main.contains("TEMPLATES_DIR"));
        assert!(main.contains("Delete this if you are using API"));
        assert!(main.contains("framework_headers"));
        assert!(main.contains("security_headers"));
        assert!(main.contains("cache_headers"));
        assert!(main.contains("cors"));
        assert!(main.contains("request_id"));
        assert!(!main.contains("MIDDLEWARE"));
        assert!(!main.contains("@router"));

        let products = fs::read_to_string(target_dir.join("src/modules/products/products.py")).unwrap();
        assert!(products.contains("http_get"));
        assert!(products.contains("CatalogAction"));
        assert!(products.contains("version=\"v1\""));
        assert!(products.contains("FusionBaseTemplate"));
        assert!(products.contains("HomePage"));
        assert!(products.contains("table_headers"));
        assert!(products.contains("@route(\"/\")"));
        assert!(products.contains("home/index.html"));

        let settings = fs::read_to_string(target_dir.join("core/settings.py")).unwrap();
        assert!(settings.contains("RELOAD"));
        assert!(settings.contains("TEMPLATES_DIR"));

        let html = fs::read_to_string(target_dir.join("templates/home/index.html")).unwrap();
        assert!(html.contains("{{ title }}"));
        assert!(html.contains(r#"{% include "fusion/components.css" %}"#));
        assert!(html.contains(r#"{% include "home/style.css" %}"#));
        assert!(html.contains("/Fusion-Framework-Transparent.png"));
        assert!(html.contains("fusion.badge"));
        assert!(html.contains("fusion.button"));
        assert!(html.contains("fusion.table"));
        assert!(html.contains("fusion.card"));
        assert!(html.contains("{{ project }}"));
        assert!(products.contains("my-app"));

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
        assert!(target_dir.join("templates/home/index.html").is_file());

        let package = fs::read_to_string(target_dir.join("package.json")).unwrap();
        assert!(!package.contains("dependencies"));
        assert!(!package.contains("devDependencies"));
        assert!(!package.contains("fusion-framework"));

        let products = fs::read_to_string(target_dir.join("src/modules/products/products.ts")).unwrap();
        assert!(products.contains("httpGet"));
        assert!(products.contains("CatalogAction"));
        assert!(products.contains("FusionBaseTemplate"));
        assert!(products.contains("HomePage"));
        assert!(products.contains("table_headers"));
        assert!(products.contains(r#"route("/")"#));

        let settings = fs::read_to_string(target_dir.join("core/settings.ts")).unwrap();
        assert!(settings.contains("RELOAD"));
        assert!(settings.contains("TEMPLATES_DIR"));

        assert!(target_dir
            .join("templates/home/Fusion-Framework-Transparent.png")
            .is_file());

        let main = fs::read_to_string(target_dir.join("main.ts")).unwrap();
        assert!(main.contains("staticFiles"));
        assert!(main.contains("TEMPLATES_DIR"));
        assert!(main.contains("Delete this if you are using API"));
        assert!(main.contains("requestId()"));
        assert!(main.find("staticFiles").unwrap() < main.find("requestId()").unwrap());
        assert!(main.find("requestId()").unwrap() < main.find("frameworkHeaders()").unwrap());
        assert!(main.contains("Delete this middleware if you are in production"));
        assert!(main.contains("frameworkHeaders"));
        assert!(main.contains("securityHeaders"));
        assert!(main.contains("cors"));
        assert!(main.contains("cacheHeaders"));
        assert!(!main.contains("MIDDLEWARE"));
        assert!(!main.contains("defaultBuiltinMiddleware"));
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
        assert!(target_dir.join("templates/home/index.html").is_file());

        let csproj = fs::read_to_string(target_dir.join("my-app.csproj")).unwrap();
        assert!(csproj.contains("net10.0"));
        assert!(!csproj.contains("PackageReference"));
        assert!(!csproj.contains("FusionFramework"));

        let products = fs::read_to_string(target_dir.join("src/modules/products/products.cs")).unwrap();
        assert!(products.contains("[HttpGet"));
        assert!(products.contains("CatalogAction"));
        assert!(products.contains("FusionBaseTemplate"));
        assert!(products.contains("HomePage"));
        assert!(products.contains("table_headers"));
        assert!(products.contains(r#"[Route("/")]"#));

        let settings = fs::read_to_string(target_dir.join("core/settings.cs")).unwrap();
        assert!(settings.contains("Reload"));
        assert!(settings.contains("TemplatesDir"));

        assert!(target_dir
            .join("templates/home/Fusion-Framework-Transparent.png")
            .is_file());

        let main = fs::read_to_string(target_dir.join("main.cs")).unwrap();
        assert!(main.contains("BuiltinMiddleware.StaticFiles"));
        assert!(main.contains("TemplatesDir"));
        assert!(main.contains("Delete this if you are using API"));
        assert!(main.find("StaticFiles").unwrap() < main.find("RequestId()").unwrap());
        assert!(main.find("RequestId()").unwrap() < main.find("FrameworkHeaders()").unwrap());
        assert!(main.contains("Delete this middleware if you are in production"));
        assert!(main.contains("FrameworkHeaders"));
        assert!(main.contains("BuiltinMiddleware.CacheHeaders"));
        assert!(main.contains("BuiltinMiddleware.RequestId"));
        assert!(main.contains("CoreSettings.Reload"));
        assert!(!main.contains("MIDDLEWARE"));

        fs::remove_dir_all(&target_dir).unwrap();
    }
}
