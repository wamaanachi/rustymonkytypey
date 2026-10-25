use std::collections::HashMap;
use ratatui::style::Color;
use serde::{ Serialize, Deserialize };

// yk this is gonna be a fully customizable color thingy, user would be able to pick and change EVERY COLOR in the tui
#[derive(Debug, Clone)]
pub struct Theme {
    // gonna be br making a lot of options
    pub name             : &'static str,
    pub border           : Color,
    pub correct          : Color,
    pub incorrect        : Color,
    pub untyped          : Color,
    pub caret_background : Color,
    pub caret_text       : Color,
    pub text             : Color,
    pub background       : Color,
    // might add more idk
}


impl Theme {
    pub fn default() -> Self {
        Self {
            // these are the colors i already had lol
            name: "Default",
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


// this is to get themes from json file
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ThemeJson {
    pub name             : String,
    pub border           : HashMap<String, u8>,
    pub correct          : HashMap<String, u8>,
    pub incorrect        : HashMap<String, u8>,
    pub untyped          : HashMap<String, u8>,
    pub caret_background : HashMap<String, u8>,
    pub caret_text       : HashMap<String, u8>,
    pub text             : HashMap<String, u8>,
    pub background       : HashMap<String, u8>,
}

impl ThemeJson {
    fn check_color(colors: Vec<HashMap<String, u8>>) -> bool {
        for color in colors {
            if color.is_empty() || color.len() == 3 {
                return false;
            }
        }

        true
    }

    // fn color_to_rgb(color: Color) -> Option<(u8, u8, u8)> {
    //     match color {
    //         Color::Rgb(r, g, b) => Some((r, g, b)),
    //         Color::Reset => None,
    //
    //         Color::Black => Some((0, 0, 0)),
    //         Color::Red => Some((187, 0, 0)),
    //         Color::Green => Some((0, 187, 0)),
    //         Color::Yellow => Some((187, 187, 0)),
    //         Color::Blue => Some((0, 0, 187)),
    //         Color::Magenta => Some((187, 0, 187)),
    //         Color::Cyan => Some((0, 187, 187)),
    //         Color::Gray => Some((187, 187, 187)),
    //         Color::DarkGray => Some((85, 85, 85)),
    //         Color::LightRed => Some((255, 85, 85)),
    //         Color::LightGreen => Some((85, 255, 85)),
    //         Color::LightYellow => Some((255, 255, 85)),
    //         Color::LightBlue => Some((85, 85, 255)),
    //         Color::LightMagenta => Some((255, 85, 255)),
    //         Color::LightCyan => Some((85, 255, 255)),
    //         Color::White => Some((255, 255, 255)),
    //
    //         Color::Indexed(index) => match index {
    //             0 => Some((0, 0, 0)),
    //             1 => Some((187, 0, 0)),
    //             2 => Some((0, 187, 0)),
    //             3 => Some((187, 187, 0)),
    //             4 => Some((0, 0, 187)),
    //             5 => Some((187, 0, 187)),
    //             6 => Some((0, 187, 187)),
    //             7 => Some((187, 187, 187)),
    //             8 => Some((85, 85, 85)),
    //             9 => Some((255, 85, 85)),
    //             10 => Some((85, 255, 85)),
    //             11 => Some((255, 255, 85)),
    //             12 => Some((85, 85, 255)),
    //             13 => Some((255, 85, 255)),
    //             14 => Some((85, 255, 255)),
    //             15 => Some((255, 255, 255)),
    //
    //
    //             16..=231 => {
    //                 let idx = index - 16;
    //                 let r = (idx / 36) % 6;
    //                 let g = (idx / 6) % 6;
    //                 let b = idx % 6;
    //
    //                 let channel_value = |c: u8| -> u8 {
    //                     if c == 0 { 0 } else { 55 + c * 40 }
    //                 };
    //
    //                 Some((channel_value(r), channel_value(g), channel_value(b)))
    //             }
    //
    //             232..=255 => {
    //                 let grey_index = index - 232;
    //                 let v = grey_index * 10 + 8;
    //                 Some((v, v, v))
    //             },
    //
    //             _ => None
    //         }
    //     }
    // }

    pub(crate) fn new(
        name             : String,
        border           : HashMap<String, u8>,
        correct          : HashMap<String, u8>,
        incorrect        : HashMap<String, u8>,
        untyped          : HashMap<String, u8>,
        caret_background : HashMap<String, u8>,
        caret_text       : HashMap<String, u8>,
        text             : HashMap<String, u8>,
        background       : HashMap<String, u8>,
    ) -> Self {
        let default: bool;

        let default_theme: Theme;

        if name.is_empty() || !Self::check_color(
            [
                border.clone(),
                correct.clone(),
                incorrect.clone(),
                untyped.clone(),
                caret_background.clone(),
                caret_text.clone(),
                text.clone(),
                background.clone(),
            ]
                .to_vec()) {
            default = true;
        } else {
            default = false
        }

        if default == true {
            default_theme = Theme::default();

            let final_name: String = {
                if name.is_empty() {
                    default_theme.name.to_owned()
                } else {
                    name
                }
            };

            // let final_border: HashMap<String, u8> = {
            //     if Self::check_color([border.clone()].to_vec()) {
            //         Self::color_to_rgb(border.clone())
            //     } else {
            //         border.to_owned()
            //     }
            // };
        }



        todo!(
            "finish this"
        )
    }
}
