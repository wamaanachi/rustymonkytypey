pub(crate) struct Config {
    theme: Option<Theme>,
    mode: Option<Mode>,
}

pub(crate) struct Theme {
    pub(crate) title: String,
    pub(crate) description: Option<String>,
    pub(crate) border: Color,
    pub(crate) correct: Color,
    pub(crate) incorrect: Color,
    pub(crate) caret_background: Color,
    pub(crate) caret_text: Color,
    pub(crate) background: Color,
}

pub(crate) struct Mode {
    pub(crate) title: String,

}

pub(crate) fn load_config() {
    todo!(
        "load config from the config file"
    )
}
