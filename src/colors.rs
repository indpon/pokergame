// get a color for the corresponding suit

use raylib::{prelude::Color};

pub fn get_color(suit: &'static str) -> Color {
    let color = match suit {
        "S" => Color::BLACK,
        "H" => Color::RED,
        "C" => Color::LIGHTBLUE,
        "D" => Color::ORANGE,
        _ => Color::WHITE,
    };

    return color;
}
