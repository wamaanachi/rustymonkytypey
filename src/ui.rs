use ratatui::{Frame, widgets::{Block, Paragraph, Wrap},};

pub fn draw(frame: &mut Frame, passage: &str, input: &str){
    let widget = Paragraph::new(crate::color_the_words::passage_line(passage, input))
        .block(Block::bordered().title("tuipe | Esc : quit"))
        .wrap(Wrap { trim: true });

    frame.render_widget(widget, frame.area());
}
