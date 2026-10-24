pub(crate) struct Config {
    theme: Option<Theme>,
    mode: Option<Mode>,
}

pub(crate) struct Theme {
    pub(crate) title: String,
    pub(crate) description: Option<String>,
    pub(crate) color: (Option<u8>, Option<u8>, Option<u8>, Option<u8>),

}

pub(crate) struct Mode {
    pub(crate) title: String,

}

pub(crate) fn load_config() {
    todo!(
        "load config from the config file"
    )
}
