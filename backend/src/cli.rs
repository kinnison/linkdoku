//! Command line interface for linkdoku
//!

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(author,version,about,long_about=None)]
pub struct Cli {
    /// Configuration file
    #[arg(
        short,
        long,
        value_name = "CONFIG",
        default_value = "linkdoku-config.yaml"
    )]
    pub config: PathBuf,
}
