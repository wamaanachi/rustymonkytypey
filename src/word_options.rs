use ratatui::{
    style::Style,
    text::{Line, Span},
};

pub const Counts: [usize; 4]  = [15, 25, 40, 50];

pub fn draw(selected: usize, theme: &crate::config::structs::theme::Theme)
    -> Line<'static>{
        
        let mut parts = vec![Span::raw["words: "]];

        for count in COUNTS {
            let mut style = Style::default().fg(theme.untyped);

            if count == selected {
                style = style
                    .fg(theme.caret_text)
                    .bg(theme.caret_background);
            }

            parts.push(Span::styled(format!(" {count} "), style));
            parts.push(Span::raw(" "));
        }

        Line::from(parts)
    }