// get a color for the corresponding suit

use raylib::{ffi::{CSSPalette, RaylibPalette}, prelude::Color};

pub fn get_color(suit: &'static str) -> Color {
    let color = match suit {
        "S" => Color::DARKGRAY,
        "H" => Color::RED,
        "C" => Color::LIGHTBLUE,
        "D" => Color::ORANGE,
        _ => Color::WHITE,
    };

    return color;
}
