

use raylib::ffi::GetRandomValue;

use crate::structs::Card;

pub fn get_random_card() -> Card {
    let rank = unsafe { GetRandomValue(2, 11) };
    let suit = unsafe { GetRandomValue(1, 4) };

    let suit_l = match suit {
        1 => "S",
        2 => "H",
        3 => "C",
        4 => "D",
        _ => panic!("Value outside not 1-4, what.")
    };

    let return_card = Card {
        suit: suit_l,
        rank: rank,
        width: 0,
        height: 0,
        x: 0,
        y: 0,
    };

    return return_card;

}
