use ratatui::{Frame, widgets::{Block, Paragraph, Wrap},};

pub fn draw(frame: &mut Frame, passage: &str, input: &str){
    let widget = Paragraph::new(format!("Type below\n\n{passage}\n\n> {input}\n\nEsc: quit"))
        .block(Block::bordered().title("tuipe"))
        .wrap(Wrap { trim: true });

    frame.render_widget(widget, frame.area());
}
