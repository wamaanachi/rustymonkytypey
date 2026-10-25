use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crate::config::structs::theme::Theme;

pub fn run(terminal: &mut ratatui::DefaultTerminal) -> io::Result<()> {
    let mut passage = crate::frontend::passage::show_passage();

    let mut input = String::new();
    let mut show_results = false;
    let mut show_review = false;
    let mut show_themes = false;
    let mut theme = Theme::default();
    let mut selected_theme = 0;

    loop {
        terminal.draw(|frame| {
            if show_themes {
                crate::design::theme::helper::draw(frame, &theme, selected_theme);
            } 
            else if show_review{
                crate::word_review::draw(frame, &theme, &passage, &input);
            }
            else if show_results {
                crate::frontend::results::draw(frame, &theme);
            } else {
                crate::frontend::ui::draw(frame, &passage, &input, &theme);
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
                KeyCode::Char('r') if show_results && !show_themes => {
                    show_review = !show_review;
                }
                KeyCode::Esc => {
                    if show_review {
                        show_review = false;
                    }
                    else if show_themes {
                        show_themes = false;
                    } else {
                        return Ok(());
                    }
                }
                KeyCode::Tab if !show_themes => {
                    passage = crate::frontend::passage::show_passage();
                    input.clear();

                    show_results = false;
                    show_review = false;
                }
                KeyCode::Enter if !show_themes && !show_review => {
                    show_results = !show_results;
                }
                KeyCode::Char(c) if !show_results && !show_themes => {
                    input.push(c);
                }
                KeyCode::Backspace if !show_results && !show_themes => {         // will this even work lol
                    input.pop();
                }
                // cause we gonna let them like go up n down in the themes page
                KeyCode::Up if show_themes => {
                    if selected_theme > 0 {
                        selected_theme -= 1;
                    }
                }
                KeyCode::Down if show_themes  => {
                    if selected_theme < 8 {
                        selected_theme += 1;
                    }
                }
                KeyCode::Left | KeyCode::Right if show_themes => {
                    if selected_theme == 0 {
                        theme = if theme.name == "Default"{
                            Theme::gruvbox()
                        }
                        else {
                            Theme::default()
                        };
                    }
                    else {

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
                _ => {}
            }
        }
    }
}
// thinkinh.thinking since so longg bruhhhh