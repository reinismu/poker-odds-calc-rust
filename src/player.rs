use crate::Card;

#[derive(Debug, Clone)]
pub struct Player {
    hand: Vec<Card>,
}

impl Player {
    pub fn new(hand: Vec<Card>) -> Player {
        Player { hand }
    }
}
