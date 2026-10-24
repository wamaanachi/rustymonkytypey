use ratatui::{
    Frame,
    style::Style,
    widgets::{Block, Paragraph},
};

pub fn draw(frame: &mut Frame, theme: &crate::theme::Theme){
    let text = format!(
        // sighs
        "Border: {}\n\
        Correct:  {}\n\
        Incorrect: {}\n\
        Untyped: {}\n\
        Caret background: {}\n\
        Caret text: {}\n\
        Text: {}\n\
        Background: {}",
        // is this the worst code ever? loll
        theme.border,
        theme.correct,
        theme.incorrect,
        theme.untyped,
        theme.caret_background,
        theme.caret_text,
        theme.text,
        theme.background,
    );

    let widget = Paragraph::new(text)
        .style(Style::default().fg(theme.text).bg(theme.background))
        .block(
            Block::bordered()
                .border_style(Style::default().fg(theme.border))
                .title("tuipe | themes | F2 / Esc:  back")
        );
    frame.render_widget(widget, frame.area());
}