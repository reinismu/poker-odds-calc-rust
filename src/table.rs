use crate::{Card, Cards, player::Player};

#[derive(Debug, Clone)]
struct Table {
    players: Vec<Player>,
    community_cards: Vec<Card>,
    dead_cards: Vec<Card>,
}

impl Table {
    fn new(player_hands: Vec<Cards>, community_cards: Vec<Card>, dead_cards: Vec<Card>) -> Table {
        Table {
            players: player_hands
                .into_iter()
                .map(|cards| Player::new(cards.cards))
                .collect(),
            community_cards,
            dead_cards,
        }
    }
}
