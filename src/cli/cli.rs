use clap::{Parser, Subcommand};

#[derive(Parser, Clone, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct CliArgs {
    /// Change Config
    #[command(subcommand)]
    config_change: Option<ConfigChange>,

    /// Theme Options
    #[arg(
        long = "theme"
    )]
    pub(crate) theme: Option<String>,

    /// Set number of words
    #[arg(
        long = "words"
    )]
    pub(crate) words: Option<usize>,


    /// Show history
    #[arg(
        long = "history"
    )]
    pub(crate) history: bool,
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
