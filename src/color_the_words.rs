use ratatui::{
    style::{Color, Style},
    text::{Line, Span},    
};

// so this will see if the letters you typed are the letters which needed to by typed
// yk will be like highlighting those if correct/wrong

pub fn passage_line(passage: &str, input: &str) -> Line<'static> {
    let mut typed = input.chars();
    let mut letters = Vec::new();

    // bruhh thisa shi is real weird
    for expected in passage.chars(){
        let color = match typed.next(){
            Some(actual) if actual == expected => Color::Green,
            Some(_) => Color::Red,
            None => Color::DarkGray,
        };

        letters.push(Span::styled(
            expected.to_string(),
            Style::default().fg(color),
        ));
    }

    Line::from(letters)
}