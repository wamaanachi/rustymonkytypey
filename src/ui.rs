use ratatui::{Frame, widgets::{Block, Paragraph, Wrap},};
use ratatui::layout::Alignment;

pub fn draw(frame: &mut Frame, passage: &str, input: &str){

    let total = passage.split_whitespace().count();

    let completed = input 
        .split_inclusive(' ')
        .filter(|word| word.ends_with(' ') && !word.trim().is_empty())
        .count();

    let completed = completed.min(total);

    let widget = Paragraph::new(crate::color_the_words::passage_line(passage, input))
        .block(Block::bordered().title(format!(
            "tuipe | {completed}/{total} words | Esc: quit"
        )))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    frame.render_widget(widget, frame.area());
}
 