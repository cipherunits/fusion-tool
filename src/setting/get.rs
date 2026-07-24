use crate::setting::FUSION_FRAMEWORK_VERSION;
use std::path::PathBuf;

pub fn get_toml() -> PathBuf {
    PathBuf::from("fusion-framework.toml")
}

pub fn get_project_root() -> PathBuf {
    std::env::current_dir().expect("Could not determine current directory")
}

pub fn project_path(path: &str) -> PathBuf {
    get_project_root().join(path)
}

pub fn config_path() -> PathBuf {
    project_path("fusion-framework.toml")
}

pub fn read_toml() -> toml::Value {
    let path = config_path();
    let content = std::fs::read_to_string(&path).expect("Could not read fusion-framework.toml");
    toml::from_str(&content).expect("Could not parse fusion-framework.toml")
}

pub fn extension() -> String {
    read_toml()
        .get("fusionframework")
        .and_then(|f| f.get("extension"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}

pub fn language() -> String {
    read_toml()
        .get("fusionframework")
        .and_then(|f| f.get("language"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}

pub fn get_version() -> String {
    read_toml()
        .get("fusionframework")
        .and_then(|f| f.get("version"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}

pub fn tool_version() -> String {
    FUSION_FRAMEWORK_VERSION.to_string()
}

pub fn version_exists_on_system() -> bool {
    read_toml()
        .get("fusionframework")
        .and_then(|f| f.get("version"))
        .and_then(|v| v.as_str())
        .map(|v| v == FUSION_FRAMEWORK_VERSION)
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_toml() {
        assert_eq!(get_toml(), PathBuf::from("fusion-framework.toml"));
    }
}
