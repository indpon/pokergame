pub struct Card {
    pub suit: &'static str,
    pub rank: &'static str,
}

pub struct Player {
    pub name: String,
    pub chips: i32,
}

pub struct Game {
    pub flop: Vec<Card>,
    pub turn: Vec<Card>,
    pub river: Vec<Card>,
}
