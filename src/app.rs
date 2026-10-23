use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

pub fn run(terminal: &mut ratatui::DefaultTerminal) -> io::Result<()>{

    let passage = crate::passage::show_passage();

    let mut input = String::new();
    let mut show_results = false;
    loop {
        terminal.draw(|frame|{
            if show_results {
                crate::results::draw(frame);
            }
            else {
                crate::ui::draw(frame, &passage, &input);
            }
        })?;

        let Event::Key(key) = event::read()? else {
            continue;
        };

        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Esc => return Ok(()),
                KeyCode::Enterb => {
                    show_results = !show_results;
                }
                KeyCode::Char(c) if !show_results => {
                    input.push(c);
                }
                KeyCode::Backspace => {         // will this even work lol
                    input.pop();
                }
                _ => {}
            }
        }
    }
}