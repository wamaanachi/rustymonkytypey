use super::theme::Theme;

use ratatui::{
    style::Style,
    text::{Line, Span, Text},    
};

// so this will see if the letters you typed are the letters which needed to by typed
// yk will be like highlighting those if correct/wrong

pub fn passage_line(passage: &str, input: &str, theme: Theme) -> Text<'static> {
    let mut typed = input.chars();
    let mut letters = Vec::new();
    let mut lines = Vec::new();
    let mut caret_shown =  false;
    // bruhh thisa shi is real weird
    for expected in passage.chars(){

        if expected == '\n' {
            lines.push(Line::from(letters));
            letters = Vec::new();

            typed.next();
            continue;
        }

        let is_untyped = typed.as_str().is_empty();

        let (characters, color) = match typed.next(){
            Some(actual) if actual == expected => (actual, theme.correct),
            Some(actual) => (actual, theme.incorrect),
            None => (expected, theme.untyped),
        };

        let mut style = Style::default().fg(color);

        if is_untyped && !caret_shown {
            style = style.bg(theme.caret_background).fg(theme.caret_text);// might changev it to white idk
            caret_shown = true;
        }

        // now like we are gonna show what theyt typed and where they are at yk
        letters.push(Span::styled(
            characters.to_string(),
            style,
        ));
    }

    for extra in typed {
        letters.push(Span::styled(
            extra.to_string(),
            Style::default().fg(theme.incorrect),
        ))
    }

    lines.push(Line::from(letters));
    Text::from(lines)
}