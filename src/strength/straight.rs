use std::collections::HashMap;

use crate::{card::Rank, Card, GameType};

pub fn get_straight(
    game_type: GameType,
    cards: &[Card],
    rank_map: &HashMap<Rank, Vec<Card>>,
) -> Option<Vec<Card>> {
    if rank_map.keys().len() < 5 {
        return None;
    }
    let is_omaha = game_type == GameType::Omaha;

    let lowest_rank = if game_type == GameType::ShortdeckHoldem {
        Rank::Six
    } else {
        Rank::Two
    };

    let mut card_match: Vec<&Card> = Vec::with_capacity(5);
    let mut card_match_omaha: Vec<Vec<&Card>> = vec![];
    for (i, card) in cards.iter().enumerate() {
        if i == 0 {
            card_match = vec![card];

            if is_omaha {
                card_match_omaha = vec![vec![card]];
            }
            continue;
        }

        let prev_card = *card_match.last().unwrap();

        if card.rank == prev_card.rank {
            if is_omaha {
                card_match_omaha.iter_mut().last().unwrap().push(card);
            }
        } else if card.rank as u8 + 1 == prev_card.rank as u8 {
            card_match.push(card);
            if is_omaha {
                card_match_omaha.push(vec![card]);
            }
            if card.rank == lowest_rank && cards[0].rank == Rank::Ace && card_match.len() == 4 {
                card_match.push(&cards[0]);
            }
        } else if card_match.len() < 5 {
            card_match = vec![card];
            if is_omaha {
                card_match_omaha = vec![vec![card]];
            }
            continue;
        }

        if card_match.len() >= 5 {
            match game_type {
                GameType::Omaha => {
                    panic!("Not yet implemented :/");
                    // TODO add logic
                }
                _ => {
                    return Some(card_match.into_iter().cloned().collect());
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::card::{Rank, Suit};

    use super::*;

    #[test]
    fn can_get_straight_flush() {
        let rank_map = hashmap! {
            Rank::Two => Card::from_cards_str("2h").unwrap(),
            Rank::Three => Card::from_cards_str("3h").unwrap(),
            Rank::Four => Card::from_cards_str("4h").unwrap(),
            Rank::Five => Card::from_cards_str("5h").unwrap(),
            Rank::Ace => Card::from_cards_str("Ac").unwrap(),
        };
        assert_eq!(
            get_straight(
                GameType::TexasHoldem,
                &Card::from_cards_str("8h7c6c5c4d").unwrap(),
                &rank_map
            ),
            Some(vec![
                Card {
                    suit: Suit::Hearts,
                    rank: Rank::Eight
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Seven
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Six
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Five
                },
                Card {
                    suit: Suit::Diamonds,
                    rank: Rank::Four
                }
            ])
        );
    }

    #[test]
    fn can_get_straight_flush_2() {
        let rank_map = hashmap! {
            Rank::Two => Card::from_cards_str("2s").unwrap(),
            Rank::Three => Card::from_cards_str("3s").unwrap(),
            Rank::Four => Card::from_cards_str("4s").unwrap(),
            Rank::Five => Card::from_cards_str("5s").unwrap(),
            Rank::King => Card::from_cards_str("Kc").unwrap(),
            Rank::Ace => Card::from_cards_str("Ad").unwrap(),
        };
        assert_eq!(
            get_straight(
                GameType::TexasHoldem,
                &Card::from_cards_str("AdKc5s4s3s2s").unwrap(),
                &rank_map
            ),
            Some(vec![
                Card {
                    suit: Suit::Spades,
                    rank: Rank::Five
                },
                Card {
                    suit: Suit::Spades,
                    rank: Rank::Four
                },
                Card {
                    suit: Suit::Spades,
                    rank: Rank::Three
                },
                Card {
                    suit: Suit::Spades,
                    rank: Rank::Two
                },
                Card {
                    suit: Suit::Diamonds,
                    rank: Rank::Ace
                }
            ])
        );
    }

    #[test]
    fn can_get_straight_flush_3() {
        let rank_map = hashmap! {
            Rank::Two => Card::from_cards_str("2s").unwrap(),
            Rank::Three => Card::from_cards_str("3s3c").unwrap(),
            Rank::Four => Card::from_cards_str("4s").unwrap(),
            Rank::Five => Card::from_cards_str("5s").unwrap(),
            Rank::King => Card::from_cards_str("Kc").unwrap(),
            Rank::Ace => Card::from_cards_str("Ad").unwrap(),
        };
        assert_eq!(
            get_straight(
                GameType::TexasHoldem,
                &Card::from_cards_str("AdKc5s4s3s3c2s").unwrap(),
                &rank_map
            ),
            Some(vec![
                Card {
                    suit: Suit::Spades,
                    rank: Rank::Five
                },
                Card {
                    suit: Suit::Spades,
                    rank: Rank::Four
                },
                Card {
                    suit: Suit::Spades,
                    rank: Rank::Three
                },
                Card {
                    suit: Suit::Spades,
                    rank: Rank::Two
                },
                Card {
                    suit: Suit::Diamonds,
                    rank: Rank::Ace
                }
            ])
        );
    }

    #[test]
    fn can_get_straight_flush_4() {
        let rank_map = hashmap! {
            Rank::Two => Card::from_cards_str("2s").unwrap(),
            Rank::Three => Card::from_cards_str("3s3c").unwrap(),
            Rank::Four => Card::from_cards_str("4s").unwrap(),
            Rank::Five => Card::from_cards_str("5s").unwrap(),
            Rank::Six => Card::from_cards_str("6c").unwrap(),
            Rank::King => Card::from_cards_str("Kc").unwrap(),
            Rank::Ace => Card::from_cards_str("Ad").unwrap(),
        };
        assert_eq!(
            get_straight(
                GameType::TexasHoldem,
                &Card::from_cards_str("AdKc6c5s4s3s3c2s").unwrap(),
                &rank_map
            ),
            Some(vec![
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Six
                },
                Card {
                    suit: Suit::Spades,
                    rank: Rank::Five
                },
                Card {
                    suit: Suit::Spades,
                    rank: Rank::Four
                },
                Card {
                    suit: Suit::Spades,
                    rank: Rank::Three
                },
                Card {
                    suit: Suit::Spades,
                    rank: Rank::Two
                }
            ])
        );
    }

    #[test]
    fn can_get_straight_flush_bad_cards() {
        let rank_map = hashmap! {
            Rank::Two => Card::from_cards_str("2h").unwrap(),
            Rank::Three => Card::from_cards_str("3h").unwrap(),
            Rank::Four => Card::from_cards_str("4h").unwrap(),
            Rank::Five => Card::from_cards_str("5h").unwrap(),
            Rank::Ace => Card::from_cards_str("Ac").unwrap(),
        };
        assert_eq!(
            get_straight(
                GameType::TexasHoldem,
                &Card::from_cards_str("Ac7c6c5c4c").unwrap(),
                &rank_map
            ),
            None
        );
    }
}
