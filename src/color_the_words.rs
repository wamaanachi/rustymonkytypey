use ratatui::{
    style::{Color, Style},
    text::{Line, Span, Text},    
};

// so this will see if the letters you typed are the letters which needed to by typed
// yk will be like highlighting those if correct/wrong

pub fn passage_line(passage: &str, input: &str) -> Text<'static> {
    let mut typed = input.chars();
    let mut letters = Vec::new();
    let mut lines = Vec::new();
    // bruhh thisa shi is real weird
    for expected in passage.chars(){

        if expected == '\n' {
            lines.push(Line::from(letters));
            letters = Vec::new();

            typed.next();
            continue;
        }
        let (characters, color) = match typed.next(){
            Some(actual) if actual == expected => (actual, Color::Green),
            Some(actual) => (actual, Color::Red),
            None => (expected, Color::DarkGray),
        };

        // now like we are gonna show what theyt typed and where they are at yk
        letters.push(Span::styled(
            characters.to_string(),
            Style::default().fg(color),
        ));
    }

    for extra in typed {
        letters.push(Span::styled(
            extra.to_string(),
            Style::default().fg(Color::Red),
        ))
    }

    lines.push(Line::from(letters));
    Text::from(lines)
}