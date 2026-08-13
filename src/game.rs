

use raylib::{ prelude::*};

use raylib_interactive::*;

use raylib::color::Color;




use crate::{ structs::Card};



pub fn render(rl: &mut RaylibHandle, thread: &RaylibThread, cards: &Vec<Card>, flop: &Vec<Card>, turn: &Vec<Card>, river: &Vec<Card>, game_state: &mut i32) {


    let rect = Rectangle::new(640.0, 600.0, 250.0, 200.0);

    let style = raylib_interactive::style::Style  {
        background_color: Color::GRAY,
        hover_color: Color::LIGHTGRAY,
        pressed_color: Color:: DARKGRAY,
        active_color: Color::GRAY,
        disabled_color: Color::GRAY,
        border_color: Color::GRAY,
        border_color_hover: Color::GRAY,
        border_color_active: Color::GRAY,
        border_color_pressed: Color::GRAY,
        text_color: Color::WHITE,
        text_color_disabled: Color::WHITE,
        text_color_hover: Color::WHITE,
        text_color_pressed: Color::WHITE,
        check_color: Color::GRAY,
        placeholder_color: Color::GRAY,
        font_size: 30,
        padding: 1.0,
        corner_radius: 1.0,
        border_thickness: 1.0
    };


    let mut start_button = Button {
        bounds: rect,
        label: "Start Game".to_string(),
        style: style,
        is_hovered: false,
        is_pressed: false,
        animation_progress: 0.0,
        enabled: true,
    };


    start_button.update(rl.get_mouse_position(), rl);
    if *game_state == 0 && start_button.is_clicked(rl) {
        // main menu
        *game_state = 1;
    }

    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::WHITE);
    if *game_state == 0 {
        start_button.draw(&mut d);
    }




    if *game_state == 1 {
        let mut card_x = 490; // base card x.

        let mut phase = "Pre-flop";

        d.draw_text(phase, 565, 300, 48, raylib::color::Color::BLACK);

        for card in cards {
            Card::render_card(card.suit, card.rank, 150, 250, card_x, 710, &mut d);
            card_x += 200;
        }
    }


}
