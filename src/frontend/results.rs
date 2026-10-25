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

    // |  |                    |   |
    let horizontal = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(86),
        Constraint::Fill(1),
    ])
    .split(frame.area());

    let center_area = horizontal[1];

    //I hope you see the vision
    //|  |                       |  |
    //|  |------------------------|  |
    //|  |                        |   |
    //|  |                        |   |
    //|  |------------------------|   |
    //|  |                        |   |
    //|  |------------------------|  |
    //|  |                        |   |
    let vertical = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(13),
        Constraint::Length(7),
        Constraint::Fill(1),
    ])
    .split(center_area);

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

    let max_wpm = stats.wpm_history.iter().copied().max().unwrap_or(0);
    let min_wpm = stats.wpm_history.iter().copied().min().unwrap_or(0);

    let graph_title = format!(
        "wpm over time | peak: {} | min: {} | avg: {:.0}",
        max_wpm, min_wpm, stats.net_wpm
    );

    //this is so that the maximum of the barchart does not touch the top border
    let max_headroom = if max_wpm > 0 {
        max_wpm + { max_wpm / 10 }.max(5)
    } else {
        100
    };

    let sparkline_wid = Sparkline::default()
        .block(
            Block::bordered()
                .border_style(Style::default().fg(theme.border))
                .title(graph_title),
        )
        .data(&stats.wpm_history)
        .max(max_headroom)
        .style(Style::default().fg(theme.correct).bg(theme.background));

    frame.render_widget(summary_widget, vertical[0]);
    frame.render_widget(sparkline_wid, vertical[1]);
}
