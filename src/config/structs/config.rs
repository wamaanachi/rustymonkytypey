use crate::config::structs::theme::Theme;

pub(crate) struct Config {
    theme: Option<Theme>,
    mode: Option<Mode>,
}

pub(crate) struct TextTheme {
    pub(crate) title: String,
    pub(crate) description: Option<String>,
    pub(crate) words: Vec<String>,
}

pub(crate) struct Mode {
    pub(crate) title: String,
}
