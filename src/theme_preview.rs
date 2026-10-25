use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, Paragraph, Wrap},
};
// the usual code 
pub fn draw(frame: &mut Frame, area: Rect, theme: &crate::config::structs::theme::Theme) {
    let mut text = crate::design::color_words::passage_line(
        "correct wrong untyped",
        "correct xxxxx ",
        theme.clone(),
    );

    text.lines.push(Line::from(""));
    text.lines.push(Line::from("WPM: 55 | Accuracy: 69%"));

    let widget = Paragraph::new(text)
        .style(Style::default().fg(theme.text).bg(theme.background))
        .block(
            Block::bordered()
                .border_style(Style::default().fg(theme.border))
                .title("Live preview")
        )
        .wrap(Wrap { trim: false });

    frame.render_widget(widget, area);
}
