use clap::Parser;
use crate::cli::cli::CliArgs;

pub mod frontend;
pub mod config;
pub mod cli;
pub mod design;

fn main() -> std::io::Result<()> {
    let args = CliArgs::parse();

    ratatui::run(frontend::app::run)
}
