pub mod init;
pub mod update;

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

    /// Upgrade fusion to the latest release
    Update,
}
