mod cli;
mod config;
mod generator;
mod model;
mod renderer;
mod storage;

use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Command::Generate { config_dir } => {
            let project_root = std::env::current_dir()?;
            generator::generate(&config_dir, &project_root)?;
        }
    }
    Ok(())
}
