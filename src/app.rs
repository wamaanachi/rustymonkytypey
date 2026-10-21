use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

pub fn run(terminal: &mut ratatui::DefaultTerminal) -> io::Result<()>{
    loop {
        terminal.draw(crate::ui::draw)?;

        let Event::Key(key) = event::read()? else {
            continue;
        };

        if key.kind == KeyEventKind::Press && key.code  == KeyCode::Esc {
            return Ok(());
        }
    }
}