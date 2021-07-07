use std::str::FromStr;

#[derive(Debug)]
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
enum Suit {
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

#[derive(Debug, PartialEq)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
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

#[derive(Debug, PartialEq)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    fn from_cards_str(s: &str) -> Result<Vec<Card>, &'static str> {
        s.chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|chunk| chunk.iter().collect::<String>())
            .map(|s| Card::from_str(&s))
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
