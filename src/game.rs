use raylib::{ prelude::*};



use crate::{ structs::Card};



pub fn render(rl: &mut RaylibHandle, thread: &RaylibThread, cards: &Vec<Card>, flop: &Vec<Card>, turn: &Vec<Card>, river: &Vec<Card>) {
    let mut d = rl.begin_drawing(thread);

    d.clear_background(Color::WHITE);
    let mut card_x = 490; // base card x.

    let mut phase = "Pre-flop";

    d.draw_text(phase, 565, 300, 48, raylib::color::Color::BLACK);

    for card in cards {
        Card::render_card(card.suit, card.rank, 150, 250, card_x, 710, &mut d);
        card_x += 200;
    }

}
