use crate::config::structs::theme::Theme;

use ratatui::{
    Frame,
    layout::Alignment,
    style::Style,
    widgets::{Block, Paragraph},
};

use crate::engine::analytics::FinalStats;

pub fn draw(frame: &mut Frame, theme: &Theme, stats: &FinalStats) {
    // ohhhh sample dataaaaaa!!!!!!!!!!!!!!!!!!
    let incorrect_chars = stats.total_chars.saturating_sub(stats.correct_chars);

    let text = format!(
        "
    \n\
        test complete!\n\n\
        wpm: {:.0}\n\
        raw wpm: {:.0}\n\
        accuracy: {:.1}%\n\n\
        Time: {:.1}s\n\
        Correct: {}\n\
        Incorrect: {}\n\n\
        R: review mistakes\n\
        Tab / Enter: new test\n\
        Esc: quit
    ",
        stats.net_wpm,
        stats.raw_wpm,
        stats.accuracy,
        stats.time_seconds,
        stats.correct_chars,
        incorrect_chars
    );

    let widget = Paragraph::new(text)
        .style(Style::default().fg(theme.text).bg(theme.background))
        .alignment(Alignment::Center)
        .block(
            Block::bordered()
                .border_style(Style::default().fg(theme.border))
                .title("tuipe | results | R: review | Enter: typing | Esc:  quit"),
        );

    frame.render_widget(widget, frame.area());
}
