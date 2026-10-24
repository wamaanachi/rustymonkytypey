use ratatui::{
    Frame,
    style::Style,
    widgets::{Block, Paragraph},
    text::Line,
};

pub fn draw(frame: &mut Frame, theme: &crate::theme::Theme, selected: usize) {
    let text = {
        let border: Color = theme.border;
        let correct: Color = theme.correct;
        let incorrect: Color = theme.incorrect;
        let untyped: Color = theme.untyped;
        let caret_bg: Color = theme.caret_background;
        let caret_txt: Color = theme.caret_text;
        let text: Color = theme.text;
        let bg: Color = theme.background;

        format!(
            // sighs
            "Border: {border}\n\
            Correct:  {correct}\n\
            Incorrect: {incorrect}\n\
            Untyped: {untyped}\n\
            Caret background: {caret_bg}\n\
            Caret text: {caret_txt}\n\
            Text: {text}\n\
            Background: {bg}",
        )
    };
    // ummn this is to put that > things beofre the options of themes you would be able to choose
    // yk how it will be used, i aint explaining

    let text: Vec<Line> = text 
        .lines()
        .enumerate()
        .map(|(index, row)|{
            let marker = if index == selected { "> " } else { "  " };

            Line::from(format!("{marker}{row}"))
        })
        .collect();

    let widget = Paragraph::new(text)
        .style(Style::default().fg(theme.text).bg(theme.background))
        .block(
            Block::bordered()
                .border_style(Style::default().fg(theme.border))
                .title("tuipe | Up/Down: select | Left/Right: colour | Esc: back")
        );
    frame.render_widget(widget, frame.area());
}