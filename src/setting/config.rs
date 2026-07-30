use serde::{Deserialize, Serialize};

/// Version of this CLI, kept in sync with Cargo.toml by the compiler
pub const FUSION_TOOL_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Version of the framework the generated projects target
pub const FUSION_FRAMEWORK_VERSION: &str = "1.0.0";

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub project: ProjectConfig,
    pub fusionframework: FrameworkConfig,
    pub tool: ToolConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectConfig {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FrameworkConfig {
    pub language: String,
    pub extension: String,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ToolConfig {
    pub version: String,
}

pub enum Language {
    Python,
    TypeScript,
    AspNetCore,
}

impl Language {
    pub fn name(&self) -> &'static str {
        match self {
            Language::Python => "python",
            Language::TypeScript => "typescript",
            Language::AspNetCore => "asp-core",
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            Language::Python => ".py",
            Language::TypeScript => ".ts",
            Language::AspNetCore => ".cs",
        }
    }
}
