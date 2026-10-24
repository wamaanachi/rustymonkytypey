use clap::{Parser, Subcommand};
use crate::config::config::{Config, Theme};

#[derive(Parser, Clone, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct CliArgs {
    /// Change Config
    #[command(subcommand)]
    config_change: ConfigChange,

    /// Theme Options
    #[arg(
        long = "theme"
    )]
    pub(crate) theme: Option<String>,

    /// Show history
    #[arg(
        long = "history"
    )]
    pub(crate) history: bool,

    /// Set number of words
    #[arg(
        long = "words"
    )]
    pub(crate) words: Option<u8>,
}

#[derive(Subcommand, Clone, Debug)]
enum ConfigChange {
    Theme {
        #[command(subcommand)]
        theme: ThemeActions
    },

    Words {
        #[command(subcommand)]
        words: Words,
    }
}

#[derive(Subcommand, Clone, Debug)]
enum ThemeActions {
    List {
        value: bool,
    },

    Set {
        theme_name: String,
    }
}

#[derive(Subcommand, Clone, Debug)]
enum Words {
    Set {
        number_of_words: u8,
    }
}
