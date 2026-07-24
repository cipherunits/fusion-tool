use crate::setting::*;
use std::fs;
use toml;

pub fn get_toml() -> std::string::String {
    "fusion-framework.toml".to_owned()
}

pub fn extension() -> std::string::String {
    let config_path = get_toml();
    let existing_content = fs::read_to_string(&config_path).unwrap();
    let existing_config: config::Config = toml::from_str(&existing_content).unwrap();
    return existing_config.fusionframework.extension;
}


pub fn language() -> std::string::String {
    let config_path = get_toml();
    let existing_content = fs::read_to_string(&config_path).unwrap();
    let existing_config: config::Config = toml::from_str(&existing_content).unwrap();
    return existing_config.fusionframework.language;
}

