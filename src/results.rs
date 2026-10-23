use ratatui::{
    Frame,
    layout::Alignmet,
    widgets::{Block, Paragraph},
};

pub fn draw(frame: &mut Frame){
    // ohhhh sample dataaaaaa!!!!!!!!!!!!!!!!!!
    let text = "\
test complete

wpm: 55
accuracy: 69%

Time: 60s
Correct: 43
Incorrect: 367

Enter: back to typing
Esc: quit";

let widget = Paragraph::new(text)
    .alignmet(Alignmet::Center)
    .block(Block::bordered().title("tuipe | results"));

    frame.render_widget(widget, frame.area());
}