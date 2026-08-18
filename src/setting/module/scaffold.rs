use super::manifest::{
    csharp_package_name, npm_package_name, python_package_name, write_manifest, ModuleBuild,
    ModuleEntry, ModuleImpl, ModuleImplLanguage, ModuleManifest, ModuleMeta, ModuleTargets,
    MANIFEST_FILE,
};
use anyhow::{bail, Context, Result};
use console::style;
use std::fs;
use std::path::Path;

pub struct ModuleInitOptions {
    pub name: String,
    pub description: String,
    pub language: ModuleImplLanguage,
    pub target_python: bool,
    pub target_typescript: bool,
    pub target_csharp: bool,
}

pub fn scaffold(target_dir: &Path, options: &ModuleInitOptions) -> Result<()> {
    if target_dir.exists() {
        let is_empty = fs::read_dir(target_dir)
            .with_context(|| format!("Could not read {}", target_dir.display()))?
            .next()
            .is_none();

        if !is_empty {
            bail!(
                "Directory '{}' already exists and is not empty.",
                target_dir.display()
            );
        }
    } else {
        fs::create_dir_all(target_dir)
            .with_context(|| format!("Could not create {}", target_dir.display()))?;
    }

    let id = options.name.to_lowercase().replace('_', "-");
    let py_package = python_package_name(&id);
    let npm_package = npm_package_name(&id);
    let cs_package = csharp_package_name(&id);

    let (target_python, target_typescript, target_csharp) = match options.language {
        ModuleImplLanguage::Python => (true, false, false),
        ModuleImplLanguage::TypeScript => (false, true, false),
        ModuleImplLanguage::CSharp => (false, false, true),
        ModuleImplLanguage::Rust => (
            options.target_python,
            options.target_typescript,
            options.target_csharp,
        ),
    };

    if !target_python && !target_typescript && !target_csharp {
        bail!("Select at least one host target language.");
    }

    let manifest = ModuleManifest {
        module: ModuleMeta {
            id: id.clone(),
            name: options.name.clone(),
            version: "0.1.0".to_string(),
            description: options.description.clone(),
            impl_: ModuleImpl {
                language: options.language,
            },
            targets: ModuleTargets {
                python: target_python,
                typescript: target_typescript,
                csharp: target_csharp,
            },
            entry: ModuleEntry {
                python: target_python.then(|| py_package.clone()),
                typescript: target_typescript.then(|| npm_package.clone()),
                csharp: target_csharp.then(|| cs_package.clone()),
            },
            build: default_build(
                options.language,
                target_python,
                target_typescript,
                target_csharp,
                &cs_package,
            ),
        },
    };

    write_manifest(&target_dir.join(MANIFEST_FILE), &manifest)?;
    report(&target_dir.join(MANIFEST_FILE));

    write(
        &target_dir.join("README.md"),
        &readme_template(
            &id,
            &py_package,
            &npm_package,
            &cs_package,
            &options.description,
            options.language,
        ),
    )?;

    write(
        &target_dir.join(".gitignore"),
        gitignore_template(options.language),
    )?;

    match options.language {
        ModuleImplLanguage::Python => scaffold_python(target_dir, &id, &py_package)?,
        ModuleImplLanguage::TypeScript => scaffold_typescript(target_dir, &id, &npm_package)?,
        ModuleImplLanguage::CSharp => scaffold_csharp(target_dir, &id, &cs_package)?,
        ModuleImplLanguage::Rust => {
            scaffold_rust(
                target_dir,
                &id,
                &py_package,
                &npm_package,
                &cs_package,
                target_python,
                target_typescript,
                target_csharp,
            )?;
        }
    }

    Ok(())
}

fn default_build(
    language: ModuleImplLanguage,
    target_python: bool,
    target_typescript: bool,
    target_csharp: bool,
    cs_package: &str,
) -> ModuleBuild {
    match language {
        ModuleImplLanguage::Python => ModuleBuild {
            python: Some("pip install -e .".to_string()),
            ..Default::default()
        },
        ModuleImplLanguage::TypeScript => ModuleBuild {
            typescript: Some("npm install && npm run build".to_string()),
            ..Default::default()
        },
        ModuleImplLanguage::CSharp => ModuleBuild {
            csharp: Some(format!("dotnet build -c Release {cs_package}/{cs_package}.csproj")),
            ..Default::default()
        },
        ModuleImplLanguage::Rust => ModuleBuild {
            rust: Some("cargo build --release".to_string()),
            python: target_python.then(|| {
                "maturin develop --release -m bindings/python/Cargo.toml".to_string()
            }),
            typescript: target_typescript.then(|| {
                "npm install --prefix bindings/node && npm run build --prefix bindings/node"
                    .to_string()
            }),
            csharp: target_csharp.then(|| {
                format!(
                    "dotnet build -c Release bindings/csharp/{cs_package}/{cs_package}.csproj"
                )
            }),
        },
    }
}

fn scaffold_python(target_dir: &Path, id: &str, package: &str) -> Result<()> {
    let pkg_dir = target_dir.join("python").join(package);
    fs::create_dir_all(&pkg_dir)?;

    write(
        &target_dir.join("pyproject.toml"),
        &format!(
            r#"[build-system]
requires = ["setuptools>=68", "wheel"]
build-backend = "setuptools.build_meta"

[project]
name = "{package}"
version = "0.1.0"
description = "Fusion module {id}"
requires-python = ">=3.10"
dependencies = []

[tool.setuptools.packages.find]
where = ["python"]
"#
        ),
    )?;

    write(
        &pkg_dir.join("__init__.py"),
        &format!(
            "\"\"\"Fusion module `{id}` — importable package for Fusion apps.\"\"\"\n\nfrom .hello import hello\n\n__all__ = [\"hello\"]\n"
        ),
    )?;

    write(
        &pkg_dir.join("hello.py"),
        &format!(
            "\"\"\"Public helpers for the `{id}` module.\"\"\"\n\n\ndef hello(name: str = \"Fusion\") -> str:\n    \"\"\"Example function — replace with your own package API.\"\"\"\n    return f\"Hello, {{name}}! (from {id})\"\n"
        ),
    )?;

    Ok(())
}

fn scaffold_typescript(target_dir: &Path, id: &str, npm_package: &str) -> Result<()> {
    let js_dir = target_dir.join("js");
    fs::create_dir_all(&js_dir)?;

    write(
        &target_dir.join("package.json"),
        &format!(
            r#"{{
  "name": "{npm_package}",
  "version": "0.1.0",
  "description": "Fusion module {id}",
  "main": "js/index.js",
  "types": "js/index.d.ts",
  "scripts": {{
    "build": "tsc -p tsconfig.json"
  }},
  "devDependencies": {{
    "typescript": "^5.6.0",
    "@types/node": "^22.0.0"
  }}
}}
"#
        ),
    )?;

    write(
        &target_dir.join("tsconfig.json"),
        r#"{
  "compilerOptions": {
    "target": "ES2020",
    "module": "CommonJS",
    "declaration": true,
    "outDir": "js",
    "rootDir": "src",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true
  },
  "include": ["src/**/*"]
}
"#,
    )?;

    let src_dir = target_dir.join("src");
    fs::create_dir_all(&src_dir)?;

    write(
        &src_dir.join("index.ts"),
        &format!(
            r#"/** Fusion module `{id}` — importable package for Fusion apps. */

export function hello(name: string = "Fusion"): string {{
  return `Hello, ${{name}}! (from {id})`;
}}
"#
        ),
    )?;

    write(
        &js_dir.join("index.js"),
        &format!(
            r#""use strict";

/** Fusion module `{id}` — importable package for Fusion apps. */

function hello(name = "Fusion") {{
  return `Hello, ${{name}}! (from {id})`;
}}

module.exports = {{ hello }};
"#
        ),
    )?;

    write(
        &js_dir.join("index.d.ts"),
        r#"export declare function hello(name?: string): string;
"#,
    )?;

    Ok(())
}

fn scaffold_csharp(target_dir: &Path, id: &str, cs_package: &str) -> Result<()> {
    let proj_dir = target_dir.join(cs_package);
    fs::create_dir_all(&proj_dir)?;

    write(
        &proj_dir.join(format!("{cs_package}.csproj")),
        &format!(
            r#"<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <TargetFramework>net8.0</TargetFramework>
    <ImplicitUsings>enable</ImplicitUsings>
    <Nullable>enable</Nullable>
    <RootNamespace>{cs_package}</RootNamespace>
    <AssemblyName>{cs_package}</AssemblyName>
    <Version>0.1.0</Version>
    <Description>Fusion module {id}</Description>
    <IsPackable>false</IsPackable>
  </PropertyGroup>
</Project>
"#
        ),
    )?;

    write(
        &proj_dir.join("Module.cs"),
        &format!(
            r#"namespace {cs_package};

/// <summary>Fusion module `{id}` — importable helpers for Fusion C# apps.</summary>
public static class Module
{{
    public static string Hello(string name = "Fusion") =>
        $"Hello, {{name}}! (from {id})";
}}
"#
        ),
    )?;

    Ok(())
}

fn scaffold_rust(
    target_dir: &Path,
    id: &str,
    py_package: &str,
    npm_package: &str,
    cs_package: &str,
    target_python: bool,
    target_typescript: bool,
    target_csharp: bool,
) -> Result<()> {
    let crate_name = format!("fusion_{}_mod", id.replace('-', "_"));
    let core_dir = target_dir.join("crates").join("core");
    fs::create_dir_all(core_dir.join("src"))?;

    write(
        &target_dir.join("Cargo.toml"),
        &format!(
            r#"[workspace]
members = ["crates/core"{python_member}{node_member}]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
"#,
            python_member = if target_python {
                ", \"bindings/python\""
            } else {
                ""
            },
            node_member = if target_typescript {
                ", \"bindings/node\""
            } else {
                ""
            },
        ),
    )?;

    write(
        &core_dir.join("Cargo.toml"),
        &format!(
            r#"[package]
name = "{crate_name}"
version.workspace = true
edition.workspace = true

[lib]
path = "src/lib.rs"
"#
        ),
    )?;

    write(
        &core_dir.join("src/lib.rs"),
        &format!(
            r#"//! Core logic for the `{id}` Fusion module.
//! Keep this crate free of PyO3 / N-API so Python, Node, and C# bindings can share it.

pub fn hello(name: &str) -> String {{
    format!("Hello, {{name}}! (from {id})")
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_hello() {{
        assert!(hello("Fusion").contains("{id}"));
    }}
}}
"#
        ),
    )?;

    if target_python {
        scaffold_rust_python(target_dir, id, py_package, &crate_name)?;
    }

    if target_typescript {
        scaffold_rust_node(target_dir, id, npm_package, &crate_name)?;
    }

    if target_csharp {
        scaffold_rust_csharp(target_dir, id, cs_package, &crate_name)?;
    }

    Ok(())
}

fn scaffold_rust_csharp(
    target_dir: &Path,
    id: &str,
    cs_package: &str,
    crate_name: &str,
) -> Result<()> {
    let bind_dir = target_dir.join("bindings").join("csharp").join(cs_package);
    fs::create_dir_all(&bind_dir)?;

    write(
        &bind_dir.join(format!("{cs_package}.csproj")),
        &format!(
            r#"<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <TargetFramework>net8.0</TargetFramework>
    <ImplicitUsings>enable</ImplicitUsings>
    <Nullable>enable</Nullable>
    <RootNamespace>{cs_package}</RootNamespace>
    <AssemblyName>{cs_package}</AssemblyName>
    <Version>0.1.0</Version>
    <Description>Fusion module {id} (C# host binding)</Description>
    <IsPackable>false</IsPackable>
  </PropertyGroup>
</Project>
"#
        ),
    )?;

    write(
        &bind_dir.join("Module.cs"),
        &format!(
            r#"namespace {cs_package};

/// <summary>
/// C# host binding for Rust module `{id}` (`{crate_name}`).
/// Replace <see cref="Hello"/> with your public API; keep shared logic in <c>crates/core</c>.
/// </summary>
public static class Module
{{
    public static string Hello(string name = "Fusion") =>
        $"Hello, {{name}}! (from {id})";
}}
"#
        ),
    )?;

    Ok(())
}

fn scaffold_rust_python(
    target_dir: &Path,
    id: &str,
    package: &str,
    crate_name: &str,
) -> Result<()> {
    let bind_dir = target_dir.join("bindings").join("python");
    let pkg_dir = bind_dir.join("python").join(package);
    fs::create_dir_all(bind_dir.join("src"))?;
    fs::create_dir_all(&pkg_dir)?;

    write(
        &bind_dir.join("Cargo.toml"),
        &format!(
            r#"[package]
name = "{package}_native"
version.workspace = true
edition.workspace = true

[lib]
name = "_native"
crate-type = ["cdylib"]
path = "src/lib.rs"

[dependencies]
{crate_name} = {{ path = "../../crates/core" }}
pyo3 = {{ version = "0.23", features = ["extension-module"] }}
"#
        ),
    )?;

    write(
        &bind_dir.join("src/lib.rs"),
        &format!(
            r#"use pyo3::prelude::*;

#[pyfunction]
#[pyo3(signature = (name="Fusion"))]
fn hello(name: &str) -> String {{
    {crate_name}::hello(name)
}}

#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {{
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    Ok(())
}}
"#
        ),
    )?;

    write(
        &bind_dir.join("pyproject.toml"),
        &format!(
            r#"[build-system]
requires = ["maturin>=1.7,<2"]
build-backend = "maturin"

[project]
name = "{package}"
version = "0.1.0"
description = "Fusion module {id} (Rust + PyO3)"
requires-python = ">=3.10"
dependencies = []

[tool.maturin]
manifest-path = "Cargo.toml"
module-name = "{package}._native"
python-source = "python"
features = ["pyo3/extension-module"]
"#
        ),
    )?;

    write(
        &pkg_dir.join("__init__.py"),
        &format!(
            "\"\"\"Fusion module `{id}` (Rust core via PyO3).\"\"\"\n\nfrom . import _native\n\n\ndef hello(name: str = \"Fusion\") -> str:\n    return _native.hello(name)\n\n\n__all__ = [\"hello\"]\n"
        ),
    )?;

    Ok(())
}

fn scaffold_rust_node(
    target_dir: &Path,
    id: &str,
    npm_package: &str,
    crate_name: &str,
) -> Result<()> {
    let bind_dir = target_dir.join("bindings").join("node");
    let js_dir = target_dir.join("js");
    fs::create_dir_all(bind_dir.join("src"))?;
    fs::create_dir_all(&js_dir)?;

    write(
        &bind_dir.join("Cargo.toml"),
        &format!(
            r#"[package]
name = "fusion-mod-{id}-node"
version.workspace = true
edition.workspace = true

[lib]
crate-type = ["cdylib"]
path = "src/lib.rs"

[dependencies]
{crate_name} = {{ path = "../../crates/core" }}
napi = {{ version = "2", default-features = false, features = ["napi4"] }}
napi-derive = "2"

[build-dependencies]
napi-build = "2"
"#
        ),
    )?;

    write(
        &bind_dir.join("build.rs"),
        r#"fn main() {
    napi_build::setup();
}
"#,
    )?;

    write(
        &bind_dir.join("src/lib.rs"),
        &format!(
            r#"#![deny(clippy::all)]

use napi_derive::napi;

#[napi]
pub fn hello(name: Option<String>) -> String {{
    {crate_name}::hello(name.as_deref().unwrap_or("Fusion"))
}}
"#
        ),
    )?;

    write(
        &bind_dir.join("package.json"),
        &format!(
            r#"{{
  "name": "{npm_package}-native",
  "version": "0.1.0",
  "main": "index.js",
  "types": "index.d.ts",
  "scripts": {{
    "build": "napi build --platform --release"
  }},
  "devDependencies": {{
    "@napi-rs/cli": "^2.18.0"
  }}
}}
"#
        ),
    )?;

    write(
        &target_dir.join("package.json"),
        &format!(
            r#"{{
  "name": "{npm_package}",
  "version": "0.1.0",
  "description": "Fusion module {id} (Rust + N-API)",
  "main": "js/index.js",
  "scripts": {{
    "build": "npm run build --prefix bindings/node"
  }}
}}
"#
        ),
    )?;

    write(
        &js_dir.join("index.js"),
        &format!(
            r#""use strict";

let native;
try {{
  native = require("../bindings/node");
}} catch {{
  native = {{
    hello: (name = "Fusion") => `Hello, ${{name}}! (from {id})`,
  }};
}}

function hello(name = "Fusion") {{
  return native.hello(name);
}}

module.exports = {{ hello }};
"#
        ),
    )?;

    write(
        &js_dir.join("index.d.ts"),
        r#"export declare function hello(name?: string): string;
"#,
    )?;

    Ok(())
}

fn readme_template(
    id: &str,
    py_package: &str,
    npm_package: &str,
    cs_package: &str,
    description: &str,
    language: ModuleImplLanguage,
) -> String {
    let usage = match language {
        ModuleImplLanguage::Python => format!(
            r#"```python
from {py_package} import hello

print(hello("world"))
```"#
        ),
        ModuleImplLanguage::TypeScript => format!(
            r#"```ts
import {{ hello }} from "{npm_package}";

console.log(hello("world"));
```"#
        ),
        ModuleImplLanguage::CSharp => format!(
            r#"```csharp
using {cs_package};

Console.WriteLine(Module.Hello("world"));
```"#
        ),
        ModuleImplLanguage::Rust => format!(
            r#"```python
from {py_package} import hello

print(hello("world"))
```

```ts
import {{ hello }} from "{npm_package}";

console.log(hello("world"));
```

```csharp
using {cs_package};

Console.WriteLine(Module.Hello("world"));
```"#
        ),
    };

    format!(
        r#"# fusion-{id}-mod

{description}

## Naming

Recommended (not required):

- Python import package: `fusion_<name>_mod` (this package: `{py_package}`)
- npm package: `fusion-<name>-mod` (this package: `{npm_package}`)
- C# project / namespace: `Fusion<Name>Mod` (this package: `{cs_package}`)

## Implementation

- Language: **{lang}**
- Manifest: `fusion.module.toml`

This is a normal library package. After `fusion add`, import it from your Fusion app.

## Usage in a Fusion app

{usage}

## Install into a Fusion app

```bash
fusion add --github YOUR_GITHUB_USERNAME/fusion-{id}-mod
```
"#,
        lang = language.as_str(),
    )
}

fn gitignore_template(language: ModuleImplLanguage) -> &'static str {
    match language {
        ModuleImplLanguage::Python => {
            r#"__pycache__/
*.py[cod]
.venv/
dist/
*.egg-info/
.pytest_cache/
"#
        }
        ModuleImplLanguage::TypeScript => {
            r#"node_modules/
js/*.js.map
*.tsbuildinfo
"#
        }
        ModuleImplLanguage::CSharp => {
            r#"bin/
obj/
"#
        }
        ModuleImplLanguage::Rust => {
            r#"target/
node_modules/
*.node
__pycache__/
.venv/
dist/
*.egg-info/
bin/
obj/
"#
        }
    }
}

fn write(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Could not create {}", parent.display()))?;
    }

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
    fn test_scaffold_python_writes_manifest_and_package() {
        let dir = std::env::temp_dir().join(format!(
            "fusion-mod-py-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));

        scaffold(
            &dir,
            &ModuleInitOptions {
                name: "example".into(),
                description: "Example module".into(),
                language: ModuleImplLanguage::Python,
                target_python: true,
                target_typescript: false,
                target_csharp: false,
            },
        )
        .unwrap();

        assert!(dir.join(MANIFEST_FILE).is_file());
        assert!(dir.join("python/fusion_example_mod/hello.py").is_file());
        assert!(!dir.join("python/fusion_example_mod/routes.py").exists());

        let init = fs::read_to_string(dir.join("python/fusion_example_mod/__init__.py")).unwrap();
        assert!(init.contains("hello"));
        assert!(!init.contains("route"));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_scaffold_rust_workspace() {
        let dir = std::env::temp_dir().join(format!(
            "fusion-mod-rs-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));

        scaffold(
            &dir,
            &ModuleInitOptions {
                name: "tokens".into(),
                description: "Token helpers".into(),
                language: ModuleImplLanguage::Rust,
                target_python: true,
                target_typescript: true,
                target_csharp: true,
            },
        )
        .unwrap();

        assert!(dir.join("crates/core/src/lib.rs").is_file());
        assert!(dir.join("bindings/python/src/lib.rs").is_file());
        assert!(!dir
            .join("bindings/python/python/fusion_tokens_mod/routes.py")
            .exists());
        assert!(dir.join("js/index.js").is_file());
        assert!(dir
            .join("bindings/csharp/FusionTokensMod/Module.cs")
            .is_file());

        let js = fs::read_to_string(dir.join("js/index.js")).unwrap();
        assert!(js.contains("function hello"));
        assert!(!js.contains("FusionBaseApi"));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_scaffold_typescript() {
        let dir = std::env::temp_dir().join(format!(
            "fusion-mod-ts-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));

        scaffold(
            &dir,
            &ModuleInitOptions {
                name: "billing".into(),
                description: "Billing module".into(),
                language: ModuleImplLanguage::TypeScript,
                target_python: false,
                target_typescript: true,
                target_csharp: false,
            },
        )
        .unwrap();

        let src = fs::read_to_string(dir.join("src/index.ts")).unwrap();
        assert!(src.contains("export function hello"));
        assert!(!src.contains("fusion-framework"));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_scaffold_csharp() {
        let dir = std::env::temp_dir().join(format!(
            "fusion-mod-cs-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));

        scaffold(
            &dir,
            &ModuleInitOptions {
                name: "security".into(),
                description: "Security helpers".into(),
                language: ModuleImplLanguage::CSharp,
                target_python: false,
                target_typescript: false,
                target_csharp: true,
            },
        )
        .unwrap();

        assert!(dir.join("FusionSecurityMod/Module.cs").is_file());
        assert!(dir.join("FusionSecurityMod/FusionSecurityMod.csproj").is_file());
        let src = fs::read_to_string(dir.join("FusionSecurityMod/Module.cs")).unwrap();
        assert!(src.contains("Hello"));
        assert!(!src.contains("FusionBaseApi"));

        let _ = fs::remove_dir_all(&dir);
    }
}
