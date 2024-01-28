use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub subcommand: Option<Commands>,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    /// Set configuration settings
    Config,
    /// Migrate to the closest AWS region or a specified one
    Migrate {
        /// Specifies the region to migrate to
        region: Option<String>,
    },
    /// Get or set current CTF
    Workon {
        /// Name of the CTF to work on
        ctf: Option<String>,
    },
    /// Sync local directory with remote one (current directory by default)
    Share {
        /// Directory to sync (defaults to current directory if not specified)
        dir: Option<String>,
    },
    /// Turn instance on
    Up,
    /// Turn instance off
    Down,
}
