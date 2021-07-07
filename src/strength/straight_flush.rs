use std::collections::HashMap;

use crate::{
    card::{Rank, Suit},
    Card, GameType,
};

pub fn get_straight_flush(
    game_type: GameType,
    suit_map: &HashMap<Suit, Vec<Card>>,
    player_cards: &Vec<Card>,
) -> Option<Vec<Card>> {
    for (_, cards) in suit_map.iter() {
        if cards.len() < 5 {
            continue;
        }
        let mut matches: Vec<Card> = vec![];
        let lowest_rank = if game_type == GameType::ShortdeckHoldem {
            Rank::Six
        } else {
            Rank::Two
        };

        for (i, card) in cards.iter().enumerate() {
            if i == 0 {
                matches = vec![*card];
                continue;
            }
            let prev_card = matches.last().unwrap();
            if card.rank as u8 + 1 == prev_card.rank as u8 {
                matches.push(*card);
                if card.rank == lowest_rank && cards[0].rank == Rank::Ace {
                    matches.push(cards[0]);
                }
                if matches.len() >= 5 {
                    match game_type {
                        GameType::Omaha => {
                            let mut omaha_matches: Vec<Card> =
                                matches.iter().cloned().rev().take(5).collect();
                            if player_cards
                                .iter()
                                .filter(|c| omaha_matches.iter().any(|oc| *oc == **c))
                                .count()
                                == 2
                            {
                                omaha_matches.sort_by(|a, b| a.rank.partial_cmp(&b.rank).unwrap());
                                return Some(omaha_matches);
                            }
                        }
                        _ => {
                            matches.sort_by(|a, b| a.rank.partial_cmp(&b.rank).unwrap());
                            return Some(matches);
                        }
                    }
                }
            } else {
                matches = vec![*card];
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::card::Rank;

    use super::*;

    #[test]
    fn can_get_straight_flush() {
        let suit_map = hashmap! {
            Suit::Hearts => vec![],
            Suit::Clubs => Card::from_cards_str("8c7c6c5c4c").unwrap(),
        };
        assert_eq!(
            get_straight_flush(GameType::TexasHoldem, &suit_map, &vec![]),
            Some(vec![
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Four
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Five
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Six
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Seven
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Eight
                }
            ])
        );
    }

    #[test]
    fn can_get_straight_flush_bad_cards() {
        let suit_map = hashmap! {
            Suit::Hearts => vec![],
            Suit::Clubs => Card::from_cards_str("Tc8c7c6c5c").unwrap(),
        };
        assert_eq!(
            get_straight_flush(GameType::TexasHoldem, &suit_map, &vec![]),
            None
        );
    }

    #[test]
    fn can_get_straight_flush_low_ace() {
        let suit_map = hashmap! {
            Suit::Hearts => vec![],
            Suit::Clubs => Card::from_cards_str("Ac5c4c3c2c").unwrap(),
        };
        assert_eq!(
            get_straight_flush(GameType::TexasHoldem, &suit_map, &vec![]),
            Some(vec![
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Two
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Three
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Four
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Five
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Ace
                }
            ])
        );
    }
    #[test]
    fn can_get_straight_flush_low_ace_short_deck() {
        let suit_map = hashmap! {
            Suit::Hearts => vec![],
            Suit::Clubs => Card::from_cards_str("Ac9c8c7c6c").unwrap(),
        };
        assert_eq!(
            get_straight_flush(GameType::ShortdeckHoldem, &suit_map, &vec![]),
            Some(vec![
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Six
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Seven
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Eight
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Nine
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Ace
                }
            ])
        );
    }
}
