use crate::config::structs::theme::Theme;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    style::Style,
    widgets::{Block, Paragraph, Sparkline},
};

use crate::engine::analytics::FinalStats;

pub fn draw(frame: &mut Frame, theme: &Theme, stats: &FinalStats) {
    // ohhhh sample dataaaaaa!!!!!!!!!!!!!!!!!!
    let incorrect_chars = stats.total_chars.saturating_sub(stats.correct_chars);

    let chunks = Layout::vertical([Constraint::Length(14), Constraint::Length(6)])
        .margin(1)
        .split(frame.area());

    let text = format!(
        "\n\
        test complete!\n\n\
        wpm: {:.0}  |  raw: {:.0}  |  accuracy: {:.1}%\n\
        time: {:.1}s  |  correct: {}  |  incorrect: {}\n\n\
        R: review mistakes  |  Tab / Enter: new test  |  Esc: quit",
        stats.net_wpm,
        stats.raw_wpm,
        stats.accuracy,
        stats.time_seconds,
        stats.correct_chars,
        incorrect_chars,
    );

    let summary_widget = Paragraph::new(text)
        .style(Style::default().fg(theme.text).bg(theme.background))
        .alignment(Alignment::Center)
        .block(
            Block::bordered()
                .border_style(Style::default().fg(theme.border))
                .title("tuipe | results | R: review | Enter: typing | Esc:  quit"),
        );

    let sparkline_wid = Sparkline::default()
        .block(
            Block::bordered()
                .border_style(Style::default().fg(theme.border))
                .title("wpm progression (speed over time"),
        )
        .data(&stats.wpm_history)
        .style(Style::default().fg(theme.correct).bg(theme.background));

    frame.render_widget(summary_widget, chunks[0]);
    frame.render_widget(sparkline_wid, chunks[1]);
}
