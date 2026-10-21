use ratatui::{Frame, widgets::{Block, Paragraph},};

pub fn draw(frame: &mut Frame, passage: &str){
    let widget = Paragraph::new(format!("Type below\n\n{passage}\n\nEsc: quit"))
        .block(Block::bordered().title("tuipe"));

    frame.render_widget(widget, frame.area());
}
