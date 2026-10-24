use ratatui::style::Color;

// yk this is gonna be a fully customizable color thingy, user would be able to pick and change EVERY COLOR in the tui
#[derive(Debug, Clone)]
pub struct Theme {
// gonna be br making a lot of options
pub border: Color,
pub correct: Color,
pub incorrect: Color,
pub untyped: Color,
pub caret_background: Color,
pub caret_text: Color,
pub text: Color,
pub background: Color,
pub name: &'static str,
// might add more idk
}

impl Theme {
    pub fn default() -> Self {
        Self{
            // these are the colors i already had lol
            border: Color::White,
            correct: Color::Green,
            incorrect: Color::Red,
            untyped: Color::DarkGray,
            caret_background: Color::Yellow,
            caret_text: Color::Black,
            text: Color::White,
            background: Color::Black,
            name: "Default",
        }
    }

    pub fn gruvbox() -> Self {
        // all these rgb's are from the gruvbox theme that i looked up online
        Self {
            name: "Gruvbox",
            border: Color::Rgb(214, 153, 33),
            correct: Color::Rgb(184, 187, 38),
            incorrect: Color::Rgb(251, 73, 52),
            untyped: Color::Rgb(146, 131, 116),
            caret_background: Color::Rgb(250, 189, 47),
            caret_text: Color::Rgb(40, 40, 40),
            text: Color::Rgb(235, 219, 178),
            background: Color::Rgb(40, 40, 40),
        }
    }
}

pub fn change_color(color: &mut Color, forward: bool){

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
    }
    else {
        (current + colors.len() - 1) % colors.len()
    };

    *color = colors[next];
}
