use crate::game;
use crate::structs::{Card, Game};

pub fn start() {
    let (mut rl, thread) = raylib::init().size(1280, 960).title("game").build();
    let mut used_cards: Vec<Card> = Vec::new();


    let cards = Card::get_hand(2);
    let flop = Card::get_hand(3);
    let turn = Card::get_hand(1);
    let river = Card::get_hand(1);


    let game = Game {
        flop,
        turn,
        river
    };

    let mut game_state: i32 = 0;
    let mut phase = "Pre-flop".to_string();
    while !rl.window_should_close() {

        game::render(&mut rl, &thread, &cards, &mut game_state, &mut phase, &game);
    }
}
