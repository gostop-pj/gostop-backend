use smartstring::alias::String;

use crate::cards::Card;

pub struct Game {
    pub id: String,
    pub dummies: Vec<Card>,
    pub players: Vec<Player>,
    pub current_player: usize,
    pub current_round: u8,
}

pub struct Player {
    pub id: String,
    pub name: String,
    pub multiplier: u8,
    pub score: u64,
}
