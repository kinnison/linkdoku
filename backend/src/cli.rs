//! Command line interface for linkdoku
//!

use std::path::PathBuf;

use clap::Parser;
use tracing::info;

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

    /// Port override
    #[arg(short, long, value_name = "PORT")]
    pub port: Option<u16>,

    /// Perform a health check instead of becoming a server
    #[arg(long)]
    pub healthcheck: bool,
}

impl Cli {
    pub fn show(&self) {
        info!("Configuration file path: {}", self.config.display());
        if let Some(port) = self.port {
            info!("CLI provided port: {port}");
        }
    }
}
