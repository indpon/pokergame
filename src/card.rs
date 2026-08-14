// this will handle a poker card, holding suit, rank, and other stuff.

use crate::{
    colors::get_color,
    random::get_random_card,
    structs::{Card, Game},
};
use raylib::prelude::*;

impl Card {
    pub fn render_card(
        suit: &'static str,
        rank: &'static str,
        width: i32,
        height: i32,
        x: i32,
        y: i32,
        d: &mut RaylibDrawHandle,
    ) {
        // render card
        let suit_color = get_color(suit);
        d.draw_rectangle(x, y, width, height, raylib::color::Color::WHITE);
        d.draw_text(&rank.to_string(), x + 60, y, 40, suit_color);
    }

    pub fn get_hand(amount: i32) -> Vec<Card> {
        let mut cards = Vec::new();

        for _ in 0..amount {
            cards.push(get_random_card());
        }

        return cards;
    }

    pub fn render_flop(game: &Game, d: &mut RaylibDrawHandle<'_>) {
        let mut flop_x = 200;
        for card in &game.flop {
            Card::render_card(card.suit, card.rank, 150, 250, flop_x, 340, d);
            flop_x += 180
        }
    }

    pub fn render_turn(game: &Game, d: &mut RaylibDrawHandle<'_>) {
        let mut flop_x = 200;
        let turn_x = 740;
        for card in &game.flop {
            Card::render_card(card.suit, card.rank, 150, 250, flop_x, 340, d);
            flop_x += 180;
        }
        for card in &game.turn {
            Card::render_card(card.suit, card.rank, 150, 250, turn_x, 340, d);
        }
    }

    pub fn render_river(game: &Game, d: &mut RaylibDrawHandle<'_>) {
        let mut flop_x = 200;
        let turn_x = 740;
        let river_x = 920;
        for card in &game.flop {
            Card::render_card(card.suit, card.rank, 150, 250, flop_x, 340, d);
            flop_x += 180;
            println!("{}", card.rank);
        }
        for card in &game.turn {
            Card::render_card(card.suit, card.rank, 150, 250, turn_x, 340, d);
        }
        for card in &game.river {
            Card::render_card(card.suit, card.rank, 150, 250, river_x, 340, d);
        }
    }
}
