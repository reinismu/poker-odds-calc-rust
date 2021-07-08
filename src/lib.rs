#[macro_use]
extern crate maplit;

use std::str::FromStr;

use card::Card;

pub mod card;
mod game;
mod player;
mod strength;
pub mod table;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameType {
    TexasHoldem,
    ShortdeckHoldem,
    Omaha,
}

impl FromStr for GameType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "shortdeck_holdem" => Ok(GameType::ShortdeckHoldem),
            "texas_holdem" => Ok(GameType::TexasHoldem),
            "omaha" => Ok(GameType::Omaha),
            _ => Err("no match"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Cards {
    pub cards: Vec<Card>,
}

impl FromStr for Cards {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = Card::from_cards_str(s);

        match cards {
            Ok(cards) => Ok(Cards { cards }),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::card::{Rank, Suit};

    use super::*;

    #[test]
    fn can_parse_cards() {
        assert_eq!(
            Cards {
                cards: vec![
                    Card {
                        rank: Rank::Ace,
                        suit: Suit::Clubs
                    },
                    Card {
                        rank: Rank::Two,
                        suit: Suit::Clubs
                    },
                ]
            },
            Cards::from_str("Ac2c").unwrap()
        );
    }
}
