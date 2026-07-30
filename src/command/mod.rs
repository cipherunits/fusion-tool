pub mod exec;
pub mod init;
pub mod update;

pub use self::exec::{exec, selected_env};
pub use self::init::init;
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

    /// Upgrade fusion to the latest release
    Update,
}
