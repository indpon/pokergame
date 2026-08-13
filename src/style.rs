
use raylib::color::Color;


pub static STYLE: raylib_interactive::style::Style  = raylib_interactive::style::Style  {
    background_color: Color::WHITE,
    hover_color: Color::LIGHTGRAY,
    pressed_color: Color:: DARKGRAY,
    active_color: Color::GRAY,
    disabled_color: Color::GRAY,
    border_color: Color::GRAY,
    border_color_hover: Color::GRAY,
    border_color_active: Color::GRAY,
    border_color_pressed: Color::GRAY,
    text_color: Color::BLACK,
    text_color_disabled: Color::WHITE,
    text_color_hover: Color::BLACK,
    text_color_pressed: Color::WHITE,
    check_color: Color::GRAY,
    placeholder_color: Color::GRAY,
    font_size: 30,
    padding: 1.0,
    corner_radius: 0.1,
    border_thickness: 1.0
};
