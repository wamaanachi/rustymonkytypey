pub(crate) mod helper;

use ratatui::style::Color;

pub fn change_color(color: &mut Color, forward: bool) {
    let colors = [
        Color::Black,
        Color::DarkGray,
        Color::White,
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
    ];

    let current = colors.iter().position(|c| c == color).unwrap_or(0);
    // wamwam
    let next = if forward {
        (current + 1) % colors.len()
    } else {
        (current + colors.len() - 1) % colors.len()
    };

    *color = colors[next];
}