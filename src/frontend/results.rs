use crate::config::structs::theme::Theme;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    style::Style,
    symbols::Marker,
    text::Line,
    widgets::{Axis, Block, Chart, Dataset, Paragraph},
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
        Constraint::Length(12),
        Constraint::Length(11),
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

    let chart_data: Vec<(f64, f64)> = stats
        .wpm_history
        .iter()
        .enumerate()
        .map(|(i, &wpm)| ((i + 1) as f64, wpm as f64))
        .collect();

    let max_wpm = stats.wpm_history.iter().copied().max().unwrap_or(0) as f64;
    let min_wpm = stats.wpm_history.iter().copied().min().unwrap_or(0) as f64;
    let total_time = stats.time_seconds.max(1.0);

    let x_bounds = [0.0, total_time];
    let mid_time = total_time / 2.0;

    let y_max = (max_wpm * 1.15).max(40.0);
    let y_bounds = [0.0, y_max];
    let y_mid = y_max / 2.0;

    let dataset = Dataset::default()
        .name("wpm")
        .marker(Marker::Braille)
        .graph_type(ratatui::widgets::GraphType::Line)
        .style(Style::default().fg(theme.correct))
        .data(&chart_data);

    let graph_title = format!(
        "wpm over time | peak: {} | min: {} | avg: {:.0}",
        max_wpm, min_wpm, stats.net_wpm
    );

    let chart_widget = Chart::new(vec![dataset])
        .block(
            Block::bordered()
                .border_style(Style::default().fg(theme.border))
                .title(graph_title),
        )
        .x_axis(
            Axis::default()
                .title(" time ")
                .style(Style::default().fg(theme.untyped))
                .bounds(x_bounds)
                .labels(vec![
                    Line::from("0s"),
                    Line::from(format!("{:.0}s", mid_time)),
                    Line::from(format!("{:.0}s", total_time)),
                ]),
        )
        .y_axis(
            Axis::default()
                .title(" wpm ")
                .style(Style::default().fg(theme.untyped))
                .bounds(y_bounds)
                .labels(vec![
                    Line::from("0"),
                    Line::from(format!("{:.0}", y_mid)),
                    Line::from(format!("{:.0}", y_max)),
                ]),
        );

    //this is so that the maximum of the barchart does not touch the top border
    frame.render_widget(summary_widget, vertical[0]);
    frame.render_widget(chart_widget, vertical[1]);
}
