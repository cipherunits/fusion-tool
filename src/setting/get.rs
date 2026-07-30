// Accessors kept as the shared read API for upcoming commands, so not every
// one of them has a caller yet.
#![allow(dead_code)]

use crate::setting::FUSION_FRAMEWORK_VERSION;
use anyhow::{Context, Result};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Name of the Fusion Framework configuration file
pub fn get_toml() -> PathBuf {
    PathBuf::from("fusion-framework.toml")
}

/// Get the current working directory
pub fn get_project_root() -> PathBuf {
    std::env::current_dir()
        .expect("Could not determine current directory")
}

/// Get a path relative to the current project root
pub fn project_path(path: &str) -> PathBuf {
    get_project_root().join(path)
}

/// Get the config path from the current project root
pub fn config_path() -> PathBuf {
    project_path("fusion-framework.toml")
}

/// Get the config path for a specific project directory
pub fn config_path_from(project_root: &Path) -> PathBuf {
    project_root.join(get_toml())
}

/// Read fusion-framework.toml from a specific project directory
pub fn read_toml_from(project_root: &Path) -> Result<toml::Value> {
    let path = config_path_from(project_root);

    let content = fs::read_to_string(&path)
        .with_context(|| {
            format!(
                "Could not read {}",
                path.display()
            )
        })?;

    let config = toml::from_str(&content)
        .context("Could not parse fusion-framework.toml")?;

    Ok(config)
}

/// Read fusion-framework.toml from the current project root
pub fn read_toml() -> Result<toml::Value> {
    let project_root = get_project_root();

    read_toml_from(&project_root)
}

/// Get the framework extension from a specific project
pub fn extension_from(project_root: &Path) -> Result<String> {
    Ok(read_toml_from(project_root)?
        .get("fusionframework")
        .and_then(|f| f.get("extension"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string())
}

/// Get the framework language from a specific project
pub fn language_from(project_root: &Path) -> Result<String> {
    Ok(read_toml_from(project_root)?
        .get("fusionframework")
        .and_then(|f| f.get("language"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string())
}

/// Get the framework version from a specific project
pub fn version_from(project_root: &Path) -> Result<String> {
    Ok(read_toml_from(project_root)?
        .get("fusionframework")
        .and_then(|f| f.get("version"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string())
}

/// Get extension from the current project
pub fn extension() -> Result<String> {
    extension_from(&get_project_root())
}

/// Get language from the current project
pub fn language() -> Result<String> {
    language_from(&get_project_root())
}

/// Get framework version from the current project
pub fn get_version() -> Result<String> {
    version_from(&get_project_root())
}

/// Get current Fusion Framework version
pub fn tool_version() -> String {
    FUSION_FRAMEWORK_VERSION.to_string()
}

/// Check if the project uses the current Fusion Framework version
pub fn version_exists_on_system(project_root: &Path) -> Result<bool> {
    Ok(version_from(project_root)? == FUSION_FRAMEWORK_VERSION)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_toml() {
        assert_eq!(
            get_toml(),
            PathBuf::from("fusion-framework.toml")
        );
    }
}