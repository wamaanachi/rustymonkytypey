use crate::{config::structs::theme::Theme, engine::TypingState};
use ratatui::layout::Alignment;
use ratatui::style::Style;
use ratatui::{
    Frame,
    widgets::{Block, Paragraph, Wrap},
};

pub fn draw(frame: &mut Frame, passage: &str, theme: &Theme, state: &TypingState) {
    let total = state.target_words.len();
    let input = state.to_input_string();
    let completed = state.current_word_idx;

    let widget = Paragraph::new(crate::design::color_words::passage_line(
        passage,
        &input,
        theme.clone(),
    ))
    .style(Style::default().fg(theme.text).bg(theme.background))
    .block(
        Block::bordered()
            .border_style(Style::default().fg(theme.border))
            .title(format!(
                "tuipe | {completed}/{total} words | F2: themes | Esc: quit"
            )),
    )
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true });

    frame.render_widget(widget, frame.area());
}
