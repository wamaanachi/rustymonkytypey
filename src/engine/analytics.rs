use std::time::{Duration, Instant};

use crate::engine::TypingState;

impl TypingState {
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

    pub fn raw_wpm(&self) -> f64 {
        let (_correct, total) = self.char_count();
        let elapsed_seconds = self.elapsed().as_secs_f64();

        if elapsed_seconds <= 0.0 {
            return 0.0;
        }

        let elapsed_min = elapsed_seconds / 60.0;
        let words_typed = (total as f64) / 5.0;

        words_typed / elapsed_min
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

        assert!((state.raw_wpm() - 4.0).abs() < f64::EPSILON);
    }
}
