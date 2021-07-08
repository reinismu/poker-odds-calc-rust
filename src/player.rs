use crate::Card;

#[derive(Debug, Clone)]
pub struct Player {
    pub hand: Vec<Card>,
}

impl Player {
    pub fn new(hand: Vec<Card>) -> Player {
        Player { hand }
    }
}
