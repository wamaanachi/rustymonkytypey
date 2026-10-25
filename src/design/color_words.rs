use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span, Text},
};

use crate::config::structs::theme::Theme;

// so this will see if the letters you typed are the letters which needed to by typed
// yk will be like highlighting those if correct/wrong
pub fn passage_line(passage: &str, input: &str, theme: Theme) -> Text<'static> {
    let mut typed = input.chars();
    let mut letters = Vec::new();
    let mut lines = Vec::new();
    let mut caret_shown = false;

    // bruhh thisa shi is real weird
    for expected in passage.chars() {
        if expected == '\n' {
            let is_untyped = typed.as_str().is_empty();
            let is_space = typed.clone().next() == Some(' ') || typed.clone().next() == Some('\n');

            let mut style = if is_space {
                typed.next();
                Style::default().fg(theme.correct)
            } else {
                Style::default().fg(theme.untyped)
            };

            if is_untyped && !caret_shown {
                style = style.bg(theme.caret_background).fg(theme.caret_text);
                caret_shown = true;
            }

            letters.push(Span::styled(" ", style));
            lines.push(Line::from(letters));
            letters = Vec::new();

            continue;
        }

        if expected == ' ' {
            while let Some(extra) = typed.clone().next() {
                if extra == ' ' {
                    break;
                }
                typed.next();

                letters.push(Span::styled(
                    extra.to_string(),
                    Style::default()
                        .fg(theme.incorrect)
                        .add_modifier(Modifier::UNDERLINED),
                ));
            }
        }

        let is_untyped = typed.as_str().is_empty();

        //Did user hit Space early to skip the rest of this word?
        let skipped_word = expected != ' ' && typed.clone().next() == Some(' ');

        let (characters, color, is_wrong) = if skipped_word {
            (expected, theme.incorrect, true)
        } else {
            let is_wrong = typed
                .clone()
                .next()
                .is_some_and(|actual| actual != expected);

            let (ch, col) = match typed.next() {
                Some(actual) if actual == expected => (actual, theme.correct),
                Some(actual) => (actual, theme.incorrect),
                None => (expected, theme.untyped),
            };
            (ch, col, is_wrong)
        };

        let mut style = Style::default().fg(color);

        if is_wrong {
            style = style.add_modifier(Modifier::UNDERLINED);
        }

        if is_untyped && !caret_shown {
            style = style.bg(theme.caret_background).fg(theme.caret_text); // might changev it to white idk
            caret_shown = true;
        }

        // now like we are gonna show what theyt typed and where they are at yk
        letters.push(Span::styled(characters.to_string(), style));
    }

    for extra in typed {
        letters.push(Span::styled(
            extra.to_string(),
            Style::default()
                .fg(theme.incorrect)
                .add_modifier(Modifier::UNDERLINED),
        ))
    }

    lines.push(Line::from(letters));
    Text::from(lines)
}
