use crate::config::structs::theme::Theme;

use ratatui::{
    Frame,
    layout::Alignment,
    widgets::{Block, Paragraph},
    style::Style,
};


pub fn draw(frame: &mut Frame, theme: &Theme) {
    // ohhhh sample dataaaaaa!!!!!!!!!!!!!!!!!!

    let text = "\
            test complete

            wpm: 55
            accuracy: 69%

            Time: 60s
            Correct: 43
            Incorrect: 367

            R: review mistakes
            Enter: back to typing
            Esc: quit";

    let widget = Paragraph::new(text)
        .style(Style::default().fg(theme.text).bg(theme.background))
        .alignment(Alignment::Center)
        .block(Block::bordered()
            .border_style(Style::default().fg(theme.border))
            .title("tuipe | results | R: review | Enter: typing | Esc:  quit")
        );


    frame.render_widget(widget, frame.area());
}

