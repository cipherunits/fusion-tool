pub mod github;
pub mod install;
pub mod manifest;
pub mod scaffold;

pub use self::github::parse_github_spec;
pub use self::install::install_module;
pub use self::manifest::{is_valid_id, ModuleImplLanguage};
pub use self::scaffold::{scaffold, ModuleInitOptions};
