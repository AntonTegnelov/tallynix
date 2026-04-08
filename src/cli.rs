use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "tallynix", about = "Generate Swedish invoices as PDF")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand)]
pub enum Command {
    /// Generate an invoice PDF from config files
    Generate {
        /// Path to config directory
        #[arg(long, default_value = "config")]
        config_dir: PathBuf,
    },
}
