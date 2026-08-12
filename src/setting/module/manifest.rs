use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub const MANIFEST_FILE: &str = "fusion.module.toml";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ModuleManifest {
    pub module: ModuleMeta,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ModuleMeta {
    pub id: String,
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub description: String,
    pub impl_: ModuleImpl,
    #[serde(default)]
    pub targets: ModuleTargets,
    #[serde(default)]
    pub entry: ModuleEntry,
    #[serde(default)]
    pub build: ModuleBuild,
}

/// Serde rename for the nested `impl` table (`impl` is a Rust keyword).
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ModuleImpl {
    pub language: ModuleImplLanguage,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ModuleImplLanguage {
    Python,
    TypeScript,
    Rust,
}

impl ModuleImplLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Python => "python",
            Self::TypeScript => "typescript",
            Self::Rust => "rust",
        }
    }

    pub fn parse(value: &str) -> Result<Self> {
        match value.to_lowercase().as_str() {
            "python" | "py" => Ok(Self::Python),
            "typescript" | "ts" | "javascript" | "js" | "node" => Ok(Self::TypeScript),
            "rust" | "rs" => Ok(Self::Rust),
            _ => bail!(
                "Unsupported module language '{}'. Available: python, typescript, rust",
                value
            ),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ModuleTargets {
    #[serde(default = "default_true")]
    pub python: bool,
    #[serde(default = "default_true")]
    pub typescript: bool,
}

impl Default for ModuleTargets {
    fn default() -> Self {
        Self {
            python: true,
            typescript: true,
        }
    }
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct ModuleEntry {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub python: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub typescript: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct ModuleBuild {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub python: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub typescript: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rust: Option<String>,
}

/// TOML uses `[module.impl]` but `impl` is a keyword — serialize via a wire format.
#[derive(Debug, Deserialize, Serialize)]
struct ManifestFile {
    module: ManifestModuleWire,
}

#[derive(Debug, Deserialize, Serialize)]
struct ManifestModuleWire {
    id: String,
    name: String,
    version: String,
    #[serde(default)]
    description: String,
    #[serde(rename = "impl")]
    impl_: ModuleImpl,
    #[serde(default)]
    targets: ModuleTargets,
    #[serde(default)]
    entry: ModuleEntry,
    #[serde(default)]
    build: ModuleBuild,
}

impl From<ManifestModuleWire> for ModuleMeta {
    fn from(wire: ManifestModuleWire) -> Self {
        Self {
            id: wire.id,
            name: wire.name,
            version: wire.version,
            description: wire.description,
            impl_: wire.impl_,
            targets: wire.targets,
            entry: wire.entry,
            build: wire.build,
        }
    }
}

impl From<&ModuleMeta> for ManifestModuleWire {
    fn from(meta: &ModuleMeta) -> Self {
        Self {
            id: meta.id.clone(),
            name: meta.name.clone(),
            version: meta.version.clone(),
            description: meta.description.clone(),
            impl_: meta.impl_.clone(),
            targets: meta.targets.clone(),
            entry: meta.entry.clone(),
            build: meta.build.clone(),
        }
    }
}

pub fn load_manifest(path: &Path) -> Result<ModuleManifest> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Could not read {}", path.display()))?;

    parse_manifest(&content)
        .with_context(|| format!("Could not parse {}", path.display()))
}

pub fn parse_manifest(content: &str) -> Result<ModuleManifest> {
    let wire: ManifestFile =
        toml::from_str(content).context("Invalid fusion.module.toml")?;

    let manifest = ModuleManifest {
        module: ModuleMeta::from(wire.module),
    };

    validate_manifest(&manifest)?;

    Ok(manifest)
}

pub fn write_manifest(path: &Path, manifest: &ModuleManifest) -> Result<()> {
    let wire = ManifestFile {
        module: ManifestModuleWire::from(&manifest.module),
    };

    let content = toml::to_string_pretty(&wire).context("Could not serialize fusion.module.toml")?;

    fs::write(path, format!("{}\n", content))
        .with_context(|| format!("Could not write {}", path.display()))?;

    Ok(())
}

pub fn validate_manifest(manifest: &ModuleManifest) -> Result<()> {
    let id = manifest.module.id.trim();

    if id.is_empty() {
        bail!("module.id must not be empty");
    }

    if !is_valid_id(id) {
        bail!(
            "module.id '{}' is invalid. Use lowercase letters, digits, and hyphens (e.g. example).",
            id
        );
    }

    if manifest.module.name.trim().is_empty() {
        bail!("module.name must not be empty");
    }

    if manifest.module.version.trim().is_empty() {
        bail!("module.version must not be empty");
    }

    if !manifest.module.targets.python && !manifest.module.targets.typescript {
        bail!("module.targets must enable at least one of python or typescript");
    }

    match manifest.module.impl_.language {
        ModuleImplLanguage::Python => {
            if !manifest.module.targets.python {
                bail!("A Python module must target python");
            }
            if manifest.module.entry.python.as_deref().unwrap_or("").is_empty() {
                bail!("module.entry.python is required for Python modules");
            }
        }
        ModuleImplLanguage::TypeScript => {
            if !manifest.module.targets.typescript {
                bail!("A TypeScript module must target typescript");
            }
            if manifest
                .module
                .entry
                .typescript
                .as_deref()
                .unwrap_or("")
                .is_empty()
            {
                bail!("module.entry.typescript is required for TypeScript modules");
            }
        }
        ModuleImplLanguage::Rust => {
            if manifest.module.targets.python
                && manifest.module.entry.python.as_deref().unwrap_or("").is_empty()
            {
                bail!("module.entry.python is required when targeting python");
            }
            if manifest.module.targets.typescript
                && manifest
                    .module
                    .entry
                    .typescript
                    .as_deref()
                    .unwrap_or("")
                    .is_empty()
            {
                bail!("module.entry.typescript is required when targeting typescript");
            }
        }
    }

    Ok(())
}

pub fn is_valid_id(id: &str) -> bool {
    let mut chars = id.chars();

    match chars.next() {
        Some(c) if c.is_ascii_lowercase() => {}
        _ => return false,
    }

    id.chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && !id.contains("--")
        && !id.ends_with('-')
}

pub fn python_package_name(id: &str) -> String {
    // Recommended: fusion_<name>_mod (e.g. fusion_jwt_mod). Not mandatory.
    format!("fusion_{}_mod", id.replace('-', "_"))
}

pub fn npm_package_name(id: &str) -> String {
    // Recommended: fusion-<name>-mod (e.g. fusion-jwt-mod). Not mandatory.
    format!("fusion-{id}-mod")
}

pub fn supports_host_language(manifest: &ModuleManifest, host_language: &str) -> bool {
    match host_language {
        "python" => manifest.module.targets.python,
        "typescript" => manifest.module.targets.typescript,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_rust() -> &'static str {
        r#"
[module]
id = "example"
name = "example"
version = "0.1.0"
description = "Example helpers"

[module.impl]
language = "rust"

[module.targets]
python = true
typescript = true

[module.entry]
python = "fusion_example_mod"
typescript = "fusion-example-mod"

[module.build]
python = "maturin develop --release"
typescript = "npm install && npm run build"
"#
    }

    #[test]
    fn test_parse_and_validate_rust_manifest() {
        let manifest = parse_manifest(sample_rust()).unwrap();

        assert_eq!(manifest.module.id, "example");
        assert_eq!(manifest.module.impl_.language, ModuleImplLanguage::Rust);
        assert!(manifest.module.targets.python);
        assert_eq!(
            manifest.module.entry.python.as_deref(),
            Some("fusion_example_mod")
        );
    }

    #[test]
    fn test_invalid_id_rejected() {
        let bad = r#"
[module]
id = "EXAMPLE"
name = "example"
version = "0.1.0"

[module.impl]
language = "python"

[module.targets]
python = true
typescript = false

[module.entry]
python = "fusion_example_mod"
"#;

        assert!(parse_manifest(bad).is_err());
    }

    #[test]
    fn test_npm_package_name() {
        assert_eq!(npm_package_name("jwt"), "fusion-jwt-mod");
        assert_eq!(npm_package_name("auth"), "fusion-auth-mod");
    }

    #[test]
    fn test_python_package_name() {
        assert_eq!(python_package_name("jwt"), "fusion_jwt_mod");
        assert_eq!(python_package_name("jwt-auth"), "fusion_jwt_auth_mod");
    }
}
