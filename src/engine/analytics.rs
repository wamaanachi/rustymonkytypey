use std::time::{Duration, Instant};

use crate::engine::TypingState;

#[derive(Debug, Clone, PartialEq)]
pub struct FinalStats {
    pub net_wpm: f64,
    pub raw_wpm: f64,
    pub accuracy: f64,
    pub time_seconds: f64,
    pub correct_chars: usize,
    pub total_chars: usize,
    pub wpm_history: Vec<u64>,
}

impl TypingState {
    pub fn stats(&self) -> FinalStats {
        let net_wpm = self.net_wpm();
        let raw_wpm = self.raw_wpm();
        let accuracy = self.accuracy();
        let time_seconds = self.elapsed().as_secs_f64();
        let (correct_chars, total_chars) = self.char_count();

        FinalStats {
            net_wpm,
            raw_wpm,
            accuracy,
            time_seconds,
            correct_chars,
            total_chars,
            wpm_history: self.wpm_history.clone(),
        }
    }

    pub fn char_count(&self) -> (usize, usize) {
        let mut total = 0;
        let mut correct = 0;
        for (target_word, typed_word) in self.target_words.iter().zip(self.typed_stuff.iter()) {
            total += typed_word.len();

            for (t, u) in target_word.iter().zip(typed_word.iter()) {
                if t == u {
                    correct += 1;
                }
            }
        }

        (correct, total)
    }

    pub fn elapsed(&self) -> Duration {
        let now = Instant::now();
        let elapsed;

        match self.started_at {
            Some(start) => {
                if let Some(ended) = self.ended_at {
                    elapsed = ended - start;
                } else {
                    elapsed = now - start;
                }
            }

            None => elapsed = Duration::new(0, 0),
        }

        elapsed
    }

    fn calculate_wpm(&self, char_count: usize) -> f64 {
        let elapsed_seconds = self.elapsed().as_secs_f64();

        if elapsed_seconds <= 0.0 {
            return 0.0;
        }

        let elapsed_min = elapsed_seconds / 60.0;
        let words_typed = (char_count as f64) / 5.0;

        words_typed / elapsed_min
    }

    pub fn raw_wpm(&self) -> f64 {
        let (_correct, total) = self.char_count();
        self.calculate_wpm(total)
    }

    pub fn net_wpm(&self) -> f64 {
        let (correct, _total) = self.char_count();
        self.calculate_wpm(correct)
    }

    pub fn accuracy(&self) -> f64 {
        let (correct, total) = self.char_count();
        if total == 0 {
            return 0.0;
        }
        (correct as f64 / total as f64) * 100.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn elapsed_time_works() {
        let words = vec!["hello".to_string()];
        let mut state = TypingState::new(words);
        let instant = Instant::now();

        assert_eq!(state.elapsed(), Duration::new(0, 0));
        state.started_at = Some(instant);
        state.ended_at = Some(instant + Duration::from_secs(5));

        assert_eq!(state.elapsed(), Duration::new(5, 0));
    }

    #[test]
    fn raw_wpm_is_correct() {
        let words = vec!["hello".to_string(), "world".to_string()];
        let mut state = TypingState::new(words);
        let ins = Instant::now();

        state.on_char('h');
        state.on_char('e');
        state.on_char('l');
        state.on_char('l');
        state.on_char('o');
        state.on_space();
        state.on_char('w');
        state.on_char('o');
        state.on_char('r');
        state.on_char('l');
        state.on_char('d');

        state.started_at = Some(ins);
        state.ended_at = Some(ins + Duration::new(30, 0));

        assert!((state.net_wpm() - 4.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_perfect_accuracy() {
        let words = vec!["hello".to_string(), "world".to_string()];
        let mut state = TypingState::new(words);

        state.on_char('h');
        state.on_char('e');
        state.on_char('l');
        state.on_char('l');
        state.on_char('o');
        state.on_space();
        state.on_char('w');
        state.on_char('o');
        state.on_char('r');
        state.on_char('l');
        state.on_char('d');

        assert_eq!(state.accuracy(), 100.0);
    }

    #[test]
    fn test_wrong_accuracy() {
        let words = vec!["hello".to_string(), "world".to_string()];
        let mut state = TypingState::new(words);

        state.on_char('h');
        state.on_char('e');
        state.on_char('l');
        state.on_char('l');
        state.on_char('o');
        state.on_space();
        state.on_char('o');
        state.on_char('w');
        state.on_char('d');
        state.on_char('k');
        state.on_char('p');

        assert_eq!(state.accuracy(), 50.0);
    }

    #[test]
    fn test_netwpm_rawwpm() {
        let words = vec!["hello".to_string(), "world".to_string()];
        let mut state = TypingState::new(words);

        state.on_char('h');
        state.on_char('e');
        state.on_char('l');
        state.on_char('l');
        state.on_char('o');
        state.on_space();
        state.on_char('o');
        state.on_char('w');
        state.on_char('d');
        state.on_char('k');
        state.on_char('p');

        assert!(state.raw_wpm() > state.net_wpm());
    }

    #[test]
    fn test_zero_elapsed_time() {
        let words = vec!["hello".to_string(), "world".to_string()];
        let state = TypingState::new(words);

        let wpm = state.net_wpm();
        assert!(wpm == 0.0);
    }

    #[test]
    fn accuracy_never_returns_nan() {
        let words = vec!["hello".to_string(), "world".to_string()];
        let state = TypingState::new(words);

        assert!(!state.accuracy().is_nan());
    }
}
