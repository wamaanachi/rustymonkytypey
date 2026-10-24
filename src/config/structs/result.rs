pub(crate) struct Result {
    pub(crate) wpm       : f32,
    pub(crate) accuracy  : f32,
    pub(crate) time      : f32,

    pub(crate) correct   : usize,
    pub(crate) incorrect : usize,
    pub(crate) metadata  : Metadata
}


pub(crate) struct Metadata {
    pub(crate) time: String,
    pub(crate) words: Vec<String>,

    pub(crate) theme: Option<String>,
}
