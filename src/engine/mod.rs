pub mod analytics;

pub struct TypingState {
    pub target_words: Vec<Vec<char>>, //collection of chars(word) in another vector(collection of words)
    pub current_word_idx: usize,      //idx of current word
    pub typed_stuff: Vec<Vec<char>>,  //stuff typed by user
    pub started_at: Option<std::time::Instant>,
    pub ended_at: Option<std::time::Instant>,
}

impl TypingState {
    pub fn new(words: Vec<String>) -> Self {
        let target_words: Vec<Vec<char>> = words.into_iter().map(|w| w.chars().collect()).collect();

        let word_count = target_words.len();

        Self {
            target_words,
            current_word_idx: 0,
            typed_stuff: vec![Vec::new(); word_count],
            started_at: None,
            ended_at: None,
        }
    }

    pub fn is_finished(&self) -> bool {
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

        let is_last_word = self.current_word_idx + 1 == self.target_words.len();
        let reached_end = self.typed_stuff[self.current_word_idx].len()
            >= self.target_words[self.current_word_idx].len();

        if is_last_word && reached_end {
            self.set_finished();
        }
    }

    pub fn on_backspace(&mut self) {
        if self.is_finished() {
            return;
        }
        if !self.typed_stuff[self.current_word_idx].is_empty() {
            self.typed_stuff[self.current_word_idx].pop();
        } else if self.current_word_idx > 0 {
            self.current_word_idx -= 1;
        }
    }

    pub fn on_space(&mut self) {
        if self.is_finished() {
            return;
        }

        if !self.typed_stuff[self.current_word_idx].is_empty() {
            self.current_word_idx += 1;
        }

        if self.current_word_idx >= self.target_words.len() {
            self.set_finished();
        }
    }

    pub fn reset(&mut self, new_words: Vec<String>) {
        self.target_words = new_words.into_iter().map(|w| w.chars().collect()).collect();
        self.typed_stuff = vec![Vec::new(); self.target_words.len()];
        self.current_word_idx = 0;
        self.started_at = None;
        self.ended_at = None;
    }

    pub fn to_input_string(&self) -> String {
        if self.typed_stuff.is_empty() {
            return String::new();
        }

        let max_idx = self
            .current_word_idx
            .min(self.typed_stuff.len().saturating_sub(1));

        let active_words = &self.typed_stuff[..=max_idx];

        active_words
            .iter()
            .map(|wc| wc.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn set_finished(&mut self) {
        self.ended_at = Some(std::time::Instant::now());
    }
}

#[cfg(test)]
mod test {
    use std::time::{Duration, Instant};

    use crate::engine::TypingState;

    #[test]
    fn space_increments_the_word() {
        let words = vec!["hello".to_string(), "world".to_string()];
        let mut state = TypingState::new(words);

        state.on_char('h');
        state.on_char('e');
        state.on_space();

        assert_eq!(state.current_word_idx, 1);

        //also let's test if pressing space again will not increment current_word_idx
        state.on_space();
        assert_eq!(state.current_word_idx, 1);

        //now, let's test if adding words, and then pressing space increments to 2
        state.on_char('w');
        state.on_char('o');
        state.on_space();

        assert_eq!(state.current_word_idx, 2);
    }

    #[test]
    fn space_without_letter_does_nothing() {
        let words = vec!["hello".to_string(), "world".to_string()];
        let mut state = TypingState::new(words);

        state.on_char('h');
        state.on_char('e');
        state.on_space();

        // let's test if pressing space again will not increment current_word_idx
        state.on_space();
        assert_eq!(state.current_word_idx, 1);

        //now, let's test if adding words, and then pressing space increments to 2
        state.on_char('w');
        state.on_char('o');
        state.on_space();

        assert_eq!(state.current_word_idx, 2);
    }
    #[test]
    fn space_sets_end_if_length_exceeds() {
        let words = vec!["hello".to_string(), "world".to_string()];
        let mut state = TypingState::new(words);

        state.on_char('h');
        state.on_char('e');
        state.on_space();

        //now, let's test if adding words, and then pressing space increments to 2
        state.on_char('w');
        state.on_char('o');
        state.on_space();

        assert_eq!(state.current_word_idx, 2);
        assert!(state.ended_at.is_some());
    }

    #[test]
    fn on_char_sets_end() {
        let words = vec!["hello".to_string()];
        let mut state = TypingState::new(words);

        state.on_char('h');
        state.on_char('e');
        state.on_char('l');
        state.on_char('l');
        state.on_char('o');

        assert!(state.ended_at.is_some());
    }

    #[test]
    fn reset_works() {
        let words = vec!["hello".to_string()];
        let mut state = TypingState::new(words);
        let instant = Instant::now();

        state.on_char('h');
        state.on_char('e');
        state.on_char('l');
        state.on_char('l');
        state.on_char('o');

        state.started_at = Some(instant);
        state.ended_at = Some(instant + Duration::new(5, 0));

        state.reset(vec!["slop".to_string()]);

        assert_eq!(state.target_words[0], vec!['s', 'l', 'o', 'p']);
        assert_eq!(state.current_word_idx, 0);
        assert!(state.started_at.is_none());
        assert!(state.ended_at.is_none());
    }

    #[test]
    fn typing_after_test_does_not_panic_raaah() {
        let words = vec!["hello".to_string()];
        let mut state = TypingState::new(words);

        state.on_char('w');
        state.on_char('0');
        state.on_char('d');

        state.reset(vec!["world".to_string()]);

        state.on_char('w');
    }
}
