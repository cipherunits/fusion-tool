pub mod add;
pub mod exec;
pub mod init;
pub mod module;
pub mod update;

pub use self::add::add;
pub use self::exec::{exec, selected_env};
pub use self::init::init;
pub use self::module::module_init;
pub use self::update::update;

use crate::setting::FUSION_TOOL_VERSION;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "fusion",
    version = FUSION_TOOL_VERSION,
    about = "Fusion Framework CLI"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new Fusion Framework project
    Init {
        /// Optional project directory
        directory: Option<String>,

        /// Programming language
        #[arg(long)]
        lang: Option<String>,

        /// Project name
        #[arg(long)]
        name: Option<String>,

        /// Project description
        #[arg(long)]
        description: Option<String>,
    },

    /// Run a command declared in an environment file
    Command {
        /// Command to run, optionally with its environment: run:stage
        name: Option<String>,

        /// Environment to take the command from
        #[arg(
            long,
            short = 'e',
            value_name = "ENV",
            conflicts_with_all = ["dev", "stage", "prod"]
        )]
        env: Option<String>,

        /// Shorthand for --env dev
        #[arg(long, conflicts_with_all = ["stage", "prod"])]
        dev: bool,

        /// Shorthand for --env stage
        #[arg(long, conflicts_with = "prod")]
        stage: bool,

        /// Shorthand for --env prod
        #[arg(long)]
        prod: bool,
    },

    /// Scaffold or manage publishable Fusion modules
    Module {
        #[command(subcommand)]
        command: ModuleCommands,
    },

    /// Add a module from GitHub into the current project
    Add {
        /// GitHub module: owner/repo or owner/repo@ref
        #[arg(long, value_name = "OWNER/REPO")]
        github: Option<String>,
    },

    /// Upgrade fusion to the latest release
    Update,
}

#[derive(Subcommand)]
pub enum ModuleCommands {
    /// Create a publishable Fusion module package
    Init {
        /// Output directory (defaults to fusion-mod-<name>)
        directory: Option<String>,

        /// Implementation language: python, typescript, rust
        #[arg(long)]
        lang: Option<String>,

        /// Module name / id
        #[arg(long)]
        name: Option<String>,

        /// Module description
        #[arg(long)]
        description: Option<String>,
    },
}
