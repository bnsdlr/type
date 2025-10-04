use ratatui::style::Color as RColor;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub enum Color {
    #[default]
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White,
    Rgb(u8, u8, u8),
    Indexed(u8),
}

impl From<Color> for RColor {
    fn from(value: Color) -> Self {
        match value {
            Color::Reset => RColor::Reset,
            Color::Black => RColor::Black,
            Color::Red => RColor::Red,
            Color::Green => RColor::Green,
            Color::Yellow => RColor::Yellow,
            Color::Blue => RColor::Blue,
            Color::Magenta => RColor::Magenta,
            Color::Cyan => RColor::Cyan,
            Color::Gray => RColor::Gray,
            Color::DarkGray => RColor::DarkGray,
            Color::LightRed => RColor::LightRed,
            Color::LightGreen => RColor::LightGreen,
            Color::LightYellow => RColor::LightYellow,
            Color::LightBlue => RColor::LightBlue,
            Color::LightMagenta => RColor::LightMagenta,
            Color::LightCyan => RColor::LightCyan,
            Color::White => RColor::White,
            Color::Rgb(r, g, b) => RColor::Rgb(r, g, b),
            Color::Indexed(i) => RColor::Indexed(i),
        }
    }
}
