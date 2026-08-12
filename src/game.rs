use raylib::prelude::*;



use crate::{card, structs::Card};



pub fn render(rl: &mut RaylibHandle, thread: &RaylibThread, cards: &Vec<Card>) {
    let mut d = rl.begin_drawing(thread);

    d.clear_background(Color::WHITE);
    let mut card_x = 490;

    for card in cards {
        Card::render_card(card.suit, card.rank, 150, 250, card_x, 710, &mut d);
        card_x += 200;
    }

}
