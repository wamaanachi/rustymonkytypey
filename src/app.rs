use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

pub fn run(terminal: &mut ratatui::DefaultTerminal) -> io::Result<()>{

    let passage = crate::passage::show_passage();

    let mut input = String::new();
    loop {
        terminal.draw(|frame| crate::ui::draw(frame, &passage, &input))?;

        let Event::Key(key) = event::read()? else {
            continue;
        };

        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Esc => return Ok(()),
                KeyCode::Char(c) => input.push(c),
                KeyCode::Backspace => {         // will this even work lol
                    input.pop();
                }
                _ => {}
            }
        }
    }
}