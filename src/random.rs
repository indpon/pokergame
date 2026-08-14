use raylib::ffi::GetRandomValue;

use crate::structs::Card;

pub fn get_random_card() -> Card {
    let rank = unsafe { GetRandomValue(2, 14) };
    let suit = unsafe { GetRandomValue(1, 4) };

    let suit_l = match suit {
        1 => "S",
        2 => "H",
        3 => "C",
        4 => "D",
        _ => panic!("Value outside not 1-4, what."),
    };

    let rank_l = match rank {
        1 => "1",
        2 => "2",
        3 => "3",
        4 => "4",
        5 => "5",
        6 => "6",
        7 => "7",
        8 => "8",
        9 => "9",
        10 => "10",
        11 => "J",
        12 => "Q",
        13 => "K",
        14 => "A",
        _ => "N",
    };

    let return_card = Card {
        suit: suit_l,
        rank: &rank_l[..],
    };

    return return_card;
}
