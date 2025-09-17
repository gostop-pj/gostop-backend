use smartstring::alias::String;

pub struct Game {
    pub id: String,
}
pub struct Player {
    pub id: String,
    pub name: String,
    pub multiplier: u8,
    pub score: u64,
}
