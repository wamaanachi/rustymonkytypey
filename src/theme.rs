use ratatui::style::Color;

// yk this is gonna be a fully customizable color thingy, user would be able to pick and change EVERY COLOR in the tui
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
// might add more idk
}

impl Theme {
    pub fn default() -> Self{
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
        }
    }
}