use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

pub fn run(terminal: &mut ratatui::DefaultTerminal) -> io::Result<()>{

    let passage = crate::passage::show_passage();

    let mut input = String::new();
    let mut show_results = false;
    let mut show_themes = false;
    let theme = crate::theme::Theme::default();
    loop {
        terminal.draw(|frame|{
            if show_themes {
                crate::themes::draw(frame, &theme);
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
                _ => {}
            }
        }
    }
}