use ratatui::{
    Frame,
    style::Style,
    widgets::{Block, Paragraph},
    text::Line,
    layout::{Constraint, Direction, Layout}
};

pub fn draw(frame: &mut Frame, theme: &crate::theme::Theme, selected: usize){
        // umm tryna make it a lil responsive idk if it'll work
    let direction = if frame.area().width >= 80 {
        Direction::Horizontal
    }
    else {
        Direction::Vertical
    };
    let areas = Layout::default()
        .direction(direction)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(40), // should work now i think
        ])
        .split(frame.area());
    let text = format!(
        // sighs
       "Preset: {}\n\
        Border: {}\n\
        Correct:  {}\n\
        Incorrect: {}\n\
        Untyped: {}\n\
        Caret background: {}\n\
        Caret text: {}\n\
        Text: {}\n\
        Background: {}",
        // is this the worst code ever? loll
        theme.name,
        theme.border,
        theme.correct,
        theme.incorrect,
        theme.untyped,
        theme.caret_background,
        theme.caret_text,
        theme.text,
        theme.background,
    );
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
    frame.render_widget(widget, areas[0]);
    crate::theme_preview::draw(frame, areas[1], theme);
}