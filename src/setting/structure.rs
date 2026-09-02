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
    framework_headers,
    security_headers,
    cors,
    cache_headers,
    request_id,
)


MIDDLEWARE = [
    request_id(),
    security_headers(),
    cors(),
    cache_headers(),

    # Other middleware
]


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
    """Root page rendered with Tera (templates/home/index.html)."""

    template = "home/index.html"

    def context(self):
        return {
            "title": "__PROJECT_NAME__",
            "message": "Your Fusion app is running.",
            "project": "__PROJECT_NAME__",
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

# Tera templates root (relative to project cwd)
TEMPLATES_DIR = settings.get("templates.dir", default="templates")

"#;

const TYPESCRIPT_MAIN: &str = r#"

/**
 * Entry point: register routes, middleware, and start the server.
 */

import "./src/modules/products/products";

import {
  FusionApp,
  frameworkHeaders,
  getSettings,
  // The four builtin factories below are exported at runtime by fusion-framework
  // but are not yet declared in its TypeScript type definitions, so they need a
  // type suppression until the package ships their types.
  // @ts-ignore - securityHeaders is runtime-exported but untyped
  securityHeaders,
  // @ts-ignore - cors is runtime-exported but untyped
  cors,
  // @ts-ignore - cacheHeaders is runtime-exported but untyped
  cacheHeaders,
  // @ts-ignore - requestId is runtime-exported but untyped
  requestId,
} from "fusion-framework";

const MIDDLEWARE = [
  frameworkHeaders(),  // Fusion identity headers (X-Powered-By / X-Framework / X-Fusion-Version)
  securityHeaders(),
  cors(),
  cacheHeaders(),
  requestId(),

  // Other middleware
];

async function main() {
  // Load fusion.<env>.json and apply core/settings overlay (RELOAD, TEMPLATES_DIR, …)
  await import("./core/settings");

  const app = new FusionApp(getSettings());

  for (const middleware of MIDDLEWARE) {
    app.use(middleware);
  }

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
      message: "Your Fusion app is running.",
      project: "__PROJECT_NAME__",
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
// Tera templates root
export const TEMPLATES_DIR = settings.get("templates.dir", "templates") ?? "templates";

settings.merge({
  reload: RELOAD,
  templates: { dir: TEMPLATES_DIR },
});
"#;

const CSHARP_MAIN: &str = r#"

// Fusion Framework entry point
using FusionFramework;

static class Program
{
    static readonly List<FusionMiddleware> MIDDLEWARE =
    [
        Middleware.FrameworkHeaders(),  // Fusion identity headers (X-Powered-By / X-Framework / X-Fusion-Version)
        BuiltinMiddleware.SecurityHeaders(),
        BuiltinMiddleware.Cors(),
        BuiltinMiddleware.CacheHeaders(),
        BuiltinMiddleware.RequestId(),

        // Other middleware
    ];

    static void Main()
    {
        Route.RegisterAll(typeof(Program).Assembly);

        SettingsStore.Current.EnsureLoaded(
            System.IO.Directory.GetCurrentDirectory()
        );

        var app = new FusionApp(SettingsStore.GetSettings());

        foreach (var middleware in MIDDLEWARE)
        {
            app.Use(middleware);
        }

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
        ["message"] = JsonValue.Create("Your Fusion app is running."),
        ["project"] = JsonValue.Create("__PROJECT_NAME__"),
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
    // Tera templates root
    public static object? TemplatesDir => SettingsStore.Current.Get("templates.dir", "templates");
}
"#;

const HOME_INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>{{ title }}</title>
  <style>{% include "home/style.css" %}</style>
</head>
<body>
  <main class="page">
    <p class="eyebrow">Fusion Framework</p>
    <h1>{{ title }}</h1>
    <p class="lead">{{ message }}</p>
    <p class="meta">Project <strong>{{ project }}</strong> · powered by Tera</p>
    <nav class="actions">
      {{<fusion.button label="Open Swagger" href="/swagger" variant="primary" />}}
      {{<fusion.link label="JSON context" href="/?format=json" />}}
    </nav>
  </main>
</body>
</html>
"#;

const HOME_STYLE_CSS: &str = r#"
:root {
  --bg: #0f1419;
  --fg: #e7ecf3;
  --muted: #9aa8b8;
  --accent: #3d8bfd;
  --card: #1a222d;
}
* { box-sizing: border-box; }
body {
  margin: 0;
  min-height: 100vh;
  font-family: "Segoe UI", system-ui, sans-serif;
  color: var(--fg);
  background:
    radial-gradient(1200px 600px at 10% -10%, #1c3a5f 0%, transparent 55%),
    radial-gradient(900px 500px at 100% 0%, #243049 0%, transparent 50%),
    var(--bg);
}
.page {
  max-width: 40rem;
  margin: 0 auto;
  padding: 4.5rem 1.5rem;
}
.eyebrow {
  letter-spacing: 0.12em;
  text-transform: uppercase;
  font-size: 0.75rem;
  color: var(--accent);
  margin: 0 0 0.75rem;
}
h1 {
  font-size: clamp(2rem, 4vw, 2.75rem);
  font-weight: 650;
  letter-spacing: -0.03em;
  margin: 0 0 0.75rem;
}
.lead {
  font-size: 1.125rem;
  color: var(--muted);
  line-height: 1.55;
  margin: 0 0 1rem;
}
.meta {
  color: var(--muted);
  font-size: 0.9rem;
  margin: 0 0 2rem;
}
.actions {
  display: flex;
  flex-wrap: wrap;
  gap: 1rem;
  align-items: center;
}
.fusion-btn {
  display: inline-block;
  padding: 0.55rem 1rem;
  border-radius: 0.4rem;
  border: none;
  cursor: pointer;
  font-weight: 600;
  text-decoration: none;
}
.fusion-btn--primary { background: var(--accent); color: #fff; }
.fusion-link { color: var(--accent); text-decoration: none; }
.fusion-link:hover { text-decoration: underline; }
"#;

/// Directories every new project starts with. `src/modules` also creates `src`.
const DIRECTORIES: [&str; 4] = [
    "core",
    "src/modules",
    "src/modules/products",
    "templates/home",
];

/// Create the starting layout of a new project:
///
/// ```text
/// ├── core
/// │   └── settings.py
/// ├── main.py
/// ├── templates
/// │   └── home
/// │       ├── index.html
/// │       └── style.css
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
        assert!(target_dir.join("pyproject.toml").is_file());

        let main = fs::read_to_string(target_dir.join("main.py")).unwrap();
        assert!(main.contains("registers @route classes"));
        assert!(main.contains("framework_headers"));
        assert!(main.contains("security_headers"));
        assert!(main.contains("request_id"));
        assert!(!main.contains("@router"));

        let products = fs::read_to_string(target_dir.join("src/modules/products/products.py")).unwrap();
        assert!(products.contains("http_get"));
        assert!(products.contains("CatalogAction"));
        assert!(products.contains("version=\"v1\""));
        assert!(products.contains("FusionBaseTemplate"));
        assert!(products.contains("HomePage"));
        assert!(products.contains("@route(\"/\")"));
        assert!(products.contains("home/index.html"));

        let settings = fs::read_to_string(target_dir.join("core/settings.py")).unwrap();
        assert!(settings.contains("RELOAD"));
        assert!(settings.contains("TEMPLATES_DIR"));

        let html = fs::read_to_string(target_dir.join("templates/home/index.html")).unwrap();
        assert!(html.contains("{{ title }}"));
        assert!(html.contains(r#"{% include "home/style.css" %}"#));
        assert!(html.contains("fusion.button"));
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
        assert!(products.contains(r#"route("/")"#));

        let settings = fs::read_to_string(target_dir.join("core/settings.ts")).unwrap();
        assert!(settings.contains("RELOAD"));
        assert!(settings.contains("TEMPLATES_DIR"));

        let main = fs::read_to_string(target_dir.join("main.ts")).unwrap();
        assert!(main.contains("frameworkHeaders"));
        assert!(main.contains("securityHeaders"));
        assert!(main.contains("cors"));
        assert!(main.contains("cacheHeaders"));
        assert!(main.contains("requestId"));
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
        assert!(products.contains(r#"[Route("/")]"#));

        let settings = fs::read_to_string(target_dir.join("core/settings.cs")).unwrap();
        assert!(settings.contains("Reload"));
        assert!(settings.contains("TemplatesDir"));

        let main = fs::read_to_string(target_dir.join("main.cs")).unwrap();
        assert!(main.contains("FrameworkHeaders"));
        assert!(main.contains("BuiltinMiddleware"));
        assert!(!main.contains("ships with none by default"));

        fs::remove_dir_all(&target_dir).unwrap();
    }
}
