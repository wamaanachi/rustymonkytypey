pub(crate) struct Result {
    pub(crate) wpm       : f32,
    pub(crate) accuracy  : f32,
    pub(crate) time      : f32,

    pub(crate) correct   : usize,
    pub(crate) incorrect : usize,
    pub(crate) metadata  : Metadata
}

impl Result {
    pub(crate) fn new(wpm: f32, accuracy: f32, time: f32, correct: usize, incorrect: usize, metadata: Metadata) -> Self {
        Self {
            wpm, accuracy, time, correct, incorrect, metadata
        }
    }

    pub(crate) fn default() -> Self {
        let wpm: f32 = 0.0;
        let accuracy: f32 = 0.0;
        let time: f32 = 0.0;
        let correct: usize = 0;
        let incorrect: usize = 0;
        let metadata: Metadata = Metadata::default();

        Self {
            wpm, accuracy, time, correct, incorrect, metadata
        }
    }
}


#[derive(Default, Clone)]
pub(crate) struct Metadata {
    pub(crate) time: String,
    pub(crate) words: Vec<String>,

    pub(crate) theme: Option<String>,
}

impl Metadata {
    pub(crate) fn default() -> Self {
        let time: String;

        let time_since_epoch = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH);

        if time_since_epoch.is_err() {
            time = time_since_epoch.unwrap().as_secs().to_string();
        } else {
            time = "".to_owned();
        }

        let words: Vec<String> = Default::default();

        let theme: Option<String> = Default::default();

        Self {
            time, words, theme
        }
    }
}
