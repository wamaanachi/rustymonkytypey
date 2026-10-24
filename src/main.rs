mod passage;
mod app;
mod ui;
mod color_the_words;
mod results;
mod theme;
mod themes;
pub mod engine;
use clap::Parser;
use crate::cli::cli::CliArgs;

pub mod config;
pub mod cli;

fn main() -> std::io::Result<()> {
    let args = CliArgs::parse();

    ratatui::run(app::run)
}