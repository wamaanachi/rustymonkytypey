use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

pub fn run(terminal: &mut ratatui::DefaultTerminal) -> io::Result<()>{

    let passage = crate::passage::show_passage();

    let mut input = String::new();
    let mut show_results = false;
    let mut show_themes = false;
    let mut theme = crate::theme::Theme::default();
    let mut selected_theme = 0;
    loop {
        terminal.draw(|frame|{
            if show_themes {
                crate::themes::draw(frame, &theme, selected_theme);
            }
            else if show_results{
                crate::results::draw(frame, &theme);
            }
            else {
                crate::ui::draw(frame, &passage, &input, &theme);
            }
    })?;

        let Event::Key(key) = event::read()? else {
            continue;
        };

        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::F(2) => {
                    show_themes = !show_themes;
                }
                KeyCode::Esc => {
                    if show_themes {
                        show_themes = false;
                    }
                    else {
                        return Ok(());
                    }
                }
                KeyCode::Enter if !show_themes => {
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
                        selected_theme -=1;
                    }
                }
                KeyCode::Down if show_themes  => {
                    if selected_theme < 7 {
                        selected_theme += 1;
                    }
                }
                KeyCode::Left | KeyCode::Right if show_themes => {

                    // just the same things in theme.rs matching it up
                    let color = match selected_theme {
                        0 => &mut theme.border,
                        1 => &mut theme.correct,
                        2 => &mut theme.incorrect,
                        3 => &mut theme.untyped,
                        4 => &mut theme.caret_background,
                        5 => &mut theme.caret_text,
                        6 => &mut theme.text,
                        _ => &mut theme.background,
                    };

                    crate::theme::change_color(color, key.code == KeyCode::Right);
                }
                _ => {}
            }
        }
    }
}