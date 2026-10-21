use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

pub fn run(terminal: &mut ratatui::DefaultTerminal) -> io::Result<()>{

    let passage = crate::passage::show_passage();
    loop {
        terminal.draw(|frame| crate::ui::draw(frame, &passage))?;

        let Event::Key(key) = event::read()? else {
            continue;
        };

        if key.kind == KeyEventKind::Press && key.code  == KeyCode::Esc {
            return Ok(());
        }
    }
}