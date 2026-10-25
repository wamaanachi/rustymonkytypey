use crate::config::structs::theme::Theme;
use crate::config::structs::result::{Metadata, Result};

use ratatui::{
    Frame,
    layout::Alignment,
    widgets::{Block, Paragraph},
    style::Style,
};


pub(crate) fn get_result() -> Result {
    let wpm: f32 = 55.0;
    let accuracy: f32 = 69.0;
    let time: f32 = 60.0;

    let correct = 43;
    let incorrect = 367;
    let metadata: Metadata = Metadata::default();

    let result = Result {
        wpm, accuracy, time, correct, incorrect, metadata
    };

    result
}


pub fn draw(frame: &mut Frame, theme: &Theme) {
    // ohhhh sample dataaaaaa!!!!!!!!!!!!!!!!!!

    let result = get_result();

    let text: String = {
        format!("\
                Test Complete!\n\n\
                \
                wpm: {}\n\
                accuracy: {}%\n\n\
                \
                Time: {}s\n\
                Correct: {}s\n\
                Incorrect: {}s\n\n\
                \
                R: review mistakes\n\
                Enter: back to typing\n\
                Esc: quit",
                result.wpm,
                result.accuracy,
                result.time,
                result.correct,
                result.incorrect
        )
    };

    let widget = Paragraph::new(text)
        .style(Style::default().fg(theme.text).bg(theme.background))
        .alignment(Alignment::Center)
        .block(Block::bordered()
            .border_style(Style::default().fg(theme.border))
            .title("tuipe | results | R: review | Enter: typing | Esc:  quit")
        );


    frame.render_widget(widget, frame.area());
}
