use crate::game;
use crate::structs::Card;

pub fn start() {
    let (mut rl, thread) = raylib::init().size(1280, 960).title("game").build();
    let cards = Card::get_hand(2);
    let flop = Card::get_hand(3);
    let turn = Card::get_hand(1);
    let river = Card::get_hand(1);
    let mut game_state: i32 = 0;
    while !rl.window_should_close() {

        game::render(&mut rl, &thread, &cards, &flop, &turn, &river, &mut game_state);
    }
}
