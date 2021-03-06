use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, EnumIter)]
pub enum Suit {
    Hearts,
    Clubs,
    Diamonds,
    Spades,
}

impl FromStr for Suit {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "h" => Ok(Suit::Hearts),
            "c" => Ok(Suit::Clubs),
            "d" => Ok(Suit::Diamonds),
            "s" => Ok(Suit::Spades),
            _ => Err("No match for Suit found"),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Eq, Hash, EnumIter)]
pub enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl FromStr for Rank {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "2" => Ok(Rank::Two),
            "3" => Ok(Rank::Three),
            "4" => Ok(Rank::Four),
            "5" => Ok(Rank::Five),
            "6" => Ok(Rank::Six),
            "7" => Ok(Rank::Seven),
            "8" => Ok(Rank::Eight),
            "9" => Ok(Rank::Nine),
            "t" => Ok(Rank::Ten),
            "j" => Ok(Rank::Jack),
            "q" => Ok(Rank::Queen),
            "k" => Ok(Rank::King),
            "a" => Ok(Rank::Ace),
            _ => Err("No match for Rank found"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn from_cards_str(s: &str) -> Result<Vec<Card>, &'static str> {
        s.chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|chunk| chunk.iter().collect::<String>())
            .map(|s| Card::from_str(&s))
            .collect()
    }

    pub fn get_all_cards() -> Vec<Card> {
        Rank::iter()
            .flat_map(|rank| Suit::iter().map(move |suit| Card { suit, rank }))
            .collect()
    }

    pub fn get_short_deck_cards() -> Vec<Card> {
        Rank::iter()
            .filter(|r| *r as u8 > 5)
            .flat_map(|rank| Suit::iter().map(move |suit| Card { suit, rank }))
            .collect()
    }
}

impl FromStr for Card {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err("Wrong size! Expected to receive 2 chars");
        }

        Ok(Card {
            rank: Rank::from_str(&s[..1])?,
            suit: Suit::from_str(&s[1..])?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_card() {
        assert_eq!(
            Card {
                rank: Rank::Ace,
                suit: Suit::Clubs
            },
            Card::from_str("Ac").unwrap()
        );
    }

    #[test]
    fn can_get_rank_value() {
        assert_eq!(Rank::Ace as u8, 14);
    }

    #[test]
    fn can_get_full_deck() {
        assert_eq!(Card::get_all_cards().len(), 52);
    }

    #[test]
    fn can_get_short_deck() {
        assert_eq!(Card::get_short_deck_cards().len(), 36);
    }
}
