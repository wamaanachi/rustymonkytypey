use clap::Parser;
use crate::cli::cli::CliArgs;

pub mod frontend;
pub mod config;
pub mod cli;
pub mod design;
pub mod engine;
mod word_review;

mod theme_preview;

fn main() -> std::io::Result<()> {
    let _args = CliArgs::parse();

    ratatui::run(frontend::app::run)
}
