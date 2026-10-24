use clap::Parser;
use crate::cli::cli::CliArgs;

pub mod config;
pub mod cli;

fn main() {
    let args = CliArgs::parse();

    return;
}
