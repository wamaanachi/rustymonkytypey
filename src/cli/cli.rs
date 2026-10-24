use clap::{Parser, Subcommand};
use crate::config::config::{Config, Theme};

#[derive(Parser, Clone, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct CliArgs {
    /// Change Config
    #[command(subcommand)]
    config_change: Option<ConfigChange>,

    /// Theme Options
    #[command(subcommand)]
    pub(crate) theme: Option<ThemeActions>,

    /// Show history
    #[arg(
        long="history"
    )]
    pub(crate) history: bool,

    /// Set number of words
    #[command(subcommand)]
    pub(crate) words: Option<WordOptions>,
}

#[derive(Subcommand, Clone, Debug)]
enum ConfigChange {
    Theme {
        #[command(subcommand)]
        theme: Option<ThemeActions>
    },

    Words {
        #[command(subcommand)]
        words: Option<WordOptions>,
    }
}

#[derive(Subcommand, Clone, Debug)]
enum ThemeActions {
    List {
        value: bool,
    },

    Set {
        theme_name: Option<String>,
    }
}

#[derive(Subcommand, Clone, Debug)]
enum WordOptions {
    Set {
        number_of_words: Option<u8>,
    }
}
