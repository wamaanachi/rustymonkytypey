use ratatui::{Frame, widgets::{Block, Paragraph, Wrap},};
use ratatui::layout::Alignment;
use ratatui::style::Style;
use crate::config::structs::theme::Theme;

pub fn draw(frame: &mut Frame, passage: &str, input: &str, theme: &Theme,) {
    let total = passage.split_whitespace().count();

    let completed = input
        .split_inclusive(' ')
        .filter(|word| word.ends_with(' ') && !word.trim().is_empty())
        .count();

    let completed = completed.min(total);

        let mut text = crate::design::color_words::passage_line(passage, input, theme.clone(),);

    text.lines.insert(0, ratatui::text::Line::from(""));

    text.lines.insert(
        0,
        ratatui::text::Line::from("Left/Right: choose before typing | Tab: restart"),
    );
text.lines.insert(0, crate::word_options::draw(total, theme));

    let widget = Paragraph::new(text)
        .style(Style::default().fg(theme.text).bg(theme.background))
        .block(Block::bordered()
            .border_style(Style::default().fg(theme.border))
            .title(format!(
                "tuipe | {completed}/{total} words | F2: themes | Esc: quit | Tab: restart"
            )))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    frame.render_widget(widget, frame.area());
}
