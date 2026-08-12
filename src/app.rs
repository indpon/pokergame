use crate::game;
use crate::structs::Card;

pub fn start() {
    let (mut rl, thread) = raylib::init().size(1280, 960).title("Hello, World").build();
    let cards = Card::get_hand(2);
    while !rl.window_should_close() {

        game::render(&mut rl, &thread, &cards);
    }
}
