use ratatui::{Frame, widgets::{Block, Paragraph},};

pub fn draw(frame: &mut frame){
    let widget = Paragraph::new("Type below\n\ Esc: quit")
        .block(Block::bordered().title("tuipe"));

    frame.render_widget(widget, frame.area());
}