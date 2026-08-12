// this will handle a poker card, holding suit, rank, and other stuff.

use crate::{colors::get_color, random::get_random_card, structs::Card};
use raylib::{ prelude::*};


use crate::colors;


impl Card {
    pub fn render_card(suit: &'static str, rank: i32, width: i32, height: i32, x: i32, y: i32, d: &mut RaylibDrawHandle) {

        // render card

        d.draw_rectangle(x, y, width, height, get_color(suit));

        if rank == 11 {
            d.draw_text("A", x + 60, y, 40, raylib::color::Color::WHITE);
        } else {
            d.draw_text(&rank.to_string(), x + 60, y, 40, raylib::color::Color::WHITE);
        }
    }

    pub fn get_hand(amount: i32) -> Vec<Card> {
        let mut cards = Vec::new();

        for _ in 0..amount {
            cards.push(get_random_card());
        }

        return cards
    }

}
