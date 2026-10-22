pub struct TypingState {
    pub target_words: Vec<Vec<char>>, //collection of chars(word) in another vector(collection of words)
    pub current_word_idx: usize,      //idx of current word
    pub typed_stuff: Vec<Vec<char>>,  //stuff typed by user
    pub started_at: Option<std::time::Instant>,
    pub ended_at: Option<std::time::Instant>,
}

impl TypingState {
    pub fn new(words: Vec<String>) -> Self {
        let targer_words: Vec<Vec<char>> = words.into_iter().map(|w| w.chars().collect()).collect();

        let word_count = targer_words.len();

        Self {
            target_words: targer_words,
            current_word_idx: 0,
            typed_stuff: vec![Vec::new(); word_count],
            started_at: None,
            ended_at: None,
        }
    }

    fn is_finished(&mut self) -> bool {
        if self.ended_at.is_some() {
            return true;
        }
        false
    }

    pub fn on_char(&mut self, c: char) {
        if self.is_finished() {
            return;
        }
        self.started_at.get_or_insert(std::time::Instant::now());
        self.typed_stuff[self.current_word_idx].push(c);
    }

    pub fn on_backspace(&mut self) {
        if self.is_finished() {
            return;
        }
        if !self.typed_stuff[self.current_word_idx].is_empty() {
            self.typed_stuff[self.current_word_idx].pop();
        }
        if self.current_word_idx > 0 {
            self.current_word_idx -= 1;
        }
    }
}
