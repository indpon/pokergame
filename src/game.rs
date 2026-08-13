use raylib::{ prelude::*};

use raylib_interactive::*;

use raylib::color::Color;

use crate::structs::Game;
use crate::{ structs::Card};
use crate::style::STYLE;

pub fn render(rl: &mut RaylibHandle, thread: &RaylibThread, cards: &Vec<Card>, game_state: &mut i32, phase: &mut String, game: &Game) {
    let background_color = Color::GRAY;
    let startrect = Rectangle::new(510.0, 600.0, 250.0, 200.0);
    let nextrect = Rectangle::new(1100.0, 800.0, 100.0, 100.0);
    let mut start_button = Button {
        bounds: startrect,
        label: "Start Game".to_string(),
        style: STYLE.clone(),
        is_hovered: false,
        is_pressed: false,
        animation_progress: 0.0,
        enabled: true,
    };
    let mut next_button = Button {
        bounds: nextrect,
        label: "next".to_string(),
        style: STYLE.clone(),
        is_hovered: false,
        is_pressed: false,
        animation_progress: 0.0,
        enabled: true,
    };

    // input handling
    start_button.update(rl.get_mouse_position(), rl);
    next_button.update(rl.get_mouse_position(), rl);
    if *game_state == 0 && start_button.is_clicked(rl) {

        *game_state = 1;
    }

    if next_button.is_clicked(rl) {
        // move to the next phase
        *phase = match phase.as_str() {
            "Pre-flop" => "Flop".to_string(),
            "Flop" => "Turn".to_string(),
            "Turn" => "River".to_string(),
            "River" => "End".to_string(),
            _ => "Error".to_string()
        };

        println!("{}", phase);
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    // rendering

    let mut d = rl.begin_drawing(thread);
    d.clear_background(background_color);

    if *game_state == 0 {
        start_button.draw(&mut d);
    }




    if *game_state == 1 {
        let mut card_x = 490; // base card x.

        d.draw_text(phase.as_str(), 565, 150, 48, raylib::color::Color::BLACK);

        for card in cards {
            Card::render_card(card.suit, card.rank, 150, 250, card_x, 710, &mut d);
            card_x += 200;
        }

        d.draw_rectangle(150, 275, 1000, 400, Color::BLACK);
        next_button.draw(&mut d);

        if phase == "Flop" { Card::render_flop(game, &mut d); }
        if phase == "Turn" { Card::render_turn(game, &mut d); }
        if phase == "River" { Card::render_river(game, &mut d); }
    }
}
