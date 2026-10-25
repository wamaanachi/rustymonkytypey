use crate::{config::structs::theme::Theme, engine::TypingState};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::widgets::TableState;
use std::io;

pub fn run(terminal: &mut ratatui::DefaultTerminal) -> io::Result<()> {
    let mut selected_count: usize = 1;
    let mut started = false;
    let mut passage =
        crate::frontend::passage::show_passage(crate::word_options::COUNTS[selected_count]);

    let words: Vec<String> = passage.split_whitespace().map(|s| s.to_string()).collect();
    let mut state = TypingState::new(words);

    let mut show_review = false;
    let mut show_themes = false;
    let mut theme = Theme::default();
    let mut selected_theme = 0;
    let mut review_state = TableState::default();

    loop {
        let is_finished = state.is_finished();

        terminal.draw(|frame| {
            if show_themes {
                crate::design::theme::helper::draw(frame, &theme, selected_theme);
            } 
            else if show_review{
                let input = state.to_input_string();
                crate::word_review::draw(frame, &theme, &passage, &input, &mut review_state,);
            }
            else if is_finished {
                crate::frontend::results::draw(frame, &theme, &state.stats());

            } else {
                crate::frontend::ui::draw(frame, &passage, &theme, &state);
            }
        })?;

        let Event::Key(key) = event::read()? else {
            continue;
        };

        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::F(2) if !show_review => {
                    show_themes = !show_themes;
                }
                KeyCode::Char('r') if is_finished && !show_themes => {
                    show_review = !show_review;

                    if show_review {
                        review_state = TableState::default();
                            // im lowk so bored bruhhh
                        let input = state.to_input_string();
                        let mistakes = crate::word_review::find_mistakes(&passage, &input);

                        if !mistakes.is_empty(){
                            review_state.select(Some(0));
                        }
                    }
                }
                KeyCode::Esc => {
                    if show_review {
                        show_review = false;
                    } else if show_themes {
                        show_themes = false;
                    } else {
                        return Ok(());
                    }
                }
                KeyCode::Tab if !show_themes => {
                    passage = crate::frontend::passage::show_passage(
                        crate::word_options::COUNTS[selected_count],
                    );
                    let new_words: Vec<String> =
                        passage.split_whitespace().map(|s| s.to_string()).collect();
                    state.reset(new_words);
                    started = false;
                    show_review = false;
                }
                KeyCode::Enter if is_finished && !show_themes && !show_review => {
                    passage = crate::frontend::passage::show_passage(
                        crate::word_options::COUNTS[selected_count],
                    );
                    let new_words: Vec<String> =
                        passage.split_whitespace().map(|s| s.to_string()).collect();
                    state.reset(new_words);
                    started = false;
                }
                // this is for the new thing i wrote in word_review which puts all the incorrectly spelt words together in a block, so yo could type them again and practice ig lol
                KeyCode::Char('p') if show_review => {
                    let input = state.to_input_string();
                    let words = crate::word_review::practice_words(&passage, &input);

                    if !words.is_empty(){
                        passage = words.join(" ");

                        state.reset(words);
                        review_state = TableState::default();
                        show_review = false;

                        started = true;
                    }
                }
                KeyCode::Char(' ') if !is_finished && !show_themes => {
                    started = true;
                    state.on_space();
                }
                KeyCode::Char('d') if show_themes => {
                    theme = Theme::default();
                }
                KeyCode::Char(c) if !is_finished && !show_themes => {
                    started = true;
                    state.on_char(c);
                }
                KeyCode::Backspace if !is_finished && !show_themes => {
                    // will this even work lol
                    // yeah now this will work
                    state.on_backspace();
                }
                // cause we gonna let them like go up n down in the themes page
                KeyCode::Up if show_themes => {
                    selected_theme = selected_theme.saturating_sub(1);
                }
                KeyCode::Down if show_themes => {
                    if selected_theme < 8 {
                        selected_theme += 1;
                    }
                }
                KeyCode::Left | KeyCode::Right if show_themes => {
                    if selected_theme == 0 {
                        theme = match (theme.name, key.code){
                            // yeah there might be some better way of doing this, but thy not just keep it simple.
                            ("Default", KeyCode::Right) => Theme::gruvbox(),
                            ("Gruvbox", KeyCode::Right) => Theme::dracula(),
                            ("Dracula", KeyCode::Left) => Theme::gruvbox(),
                            ("Gruvbox", KeyCode::Left) => Theme::default(),
                            ("Default", KeyCode::Left) => Theme::dracula(),
                            _ => Theme::default(),
                        };
                    } else {
                        // just the same things in theme.rs matching it up
                        let color = match selected_theme {
                            1 => &mut theme.border,
                            2 => &mut theme.correct,
                            3 => &mut theme.incorrect,
                            4 => &mut theme.untyped,
                            5 => &mut theme.caret_background,
                            6 => &mut theme.caret_text,
                            7 => &mut theme.text,
                            _ => &mut theme.background,
                        };

                        crate::design::theme::change_color(color, key.code == KeyCode::Right);

                        theme.name = "Custom";
                    }
                }
                KeyCode::Left | KeyCode::Right
                    if !show_themes && !is_finished && !show_review && !started =>
                {
                    if key.code == KeyCode::Left {
                        selected_count = selected_count.saturating_sub(1);
                    } else {
                        selected_count = (selected_count + 1).min(3);
                    }

                    passage = crate::frontend::passage::show_passage(
                        crate::word_options::COUNTS[selected_count],
                    );
                    let new_words: Vec<String> =
                        passage.split_whitespace().map(|s| s.to_string()).collect();
                    state.reset(new_words);
                } 
                KeyCode::Up if show_review => {
                    if let Some(selected) = review_state.selected(){
                        review_state.select(Some(selected.saturating_sub(1)));
                    }
                }

                KeyCode::Down if show_review => {
                    let input = state.to_input_string();
                    let total = crate::word_review::find_mistakes(&passage, &input).len();

                    if let Some(selected) = review_state.selected(){
                        if selected + 1 < total {
                            review_state.select(Some(selected + 1));
                        }
                    }
                }

                KeyCode::Home if show_review => {
                    if review_state.selected().is_some(){
                        review_state.select(Some(0));
                    }
                }

                KeyCode::End if show_review => {
                    let input = state.to_input_string();
                    let total = crate::word_review::find_mistakes(&passage, &input).len();

                    if total > 0 {
                        review_state.select(Some(total - 1));
                    }
                }
                _ => {}
            }
        }
    }
}