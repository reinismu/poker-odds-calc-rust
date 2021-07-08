use std::collections::HashMap;

use crate::{card::Rank, Card, GameType};

pub fn get_full_house(
    game_type: GameType,
    rank_map: HashMap<Rank, Vec<Card>>,
    player_cards: &[Card],
) -> Option<Vec<Card>> {
    let mut three_of_a_kinds: Vec<&[Card]> = vec![];
    let mut pairs: Vec<&[Card]> = vec![];

    for (_, rank_cards) in rank_map.iter() {
        let ln = rank_cards.len();
        if ln == 2 {
            pairs.push(rank_cards);
        } else if ln == 3 {
            three_of_a_kinds.push(rank_cards);
        }
    }
    if three_of_a_kinds.len() * 10 + pairs.len() < 11 {
        return None;
    }

    match game_type {
        GameType::Omaha => {
            three_of_a_kinds.sort_by(|a, b| b[0].rank.partial_cmp(&a[0].rank).unwrap());
            let mut all_combinations: Vec<&[Card]> =
                three_of_a_kinds.iter().chain(&pairs).cloned().collect();
            all_combinations.sort_by(|a, b| a[0].rank.partial_cmp(&b[0].rank).unwrap());

            for three_of_a_kind in three_of_a_kinds.into_iter() {
                for trips_or_pair in all_combinations.iter() {
                    if three_of_a_kind[0] != trips_or_pair[0] {
                        for (i, _) in trips_or_pair.iter().enumerate() {
                            let this_combo: Vec<Card> = three_of_a_kind
                                .iter()
                                .chain(trips_or_pair.iter().skip(i).take(2))
                                .cloned()
                                .collect();
                            if player_cards
                                .iter()
                                .filter(|pc| this_combo.iter().any(|c| *c == **pc))
                                .count()
                                == 2
                            {
                                return Some(this_combo);
                            }
                        }
                    }
                }
            }
        }
        _ => {
            if three_of_a_kinds.len() > 1 {
                return Some(
                    three_of_a_kinds[0]
                        .iter()
                        .chain(three_of_a_kinds[1].iter().take(2))
                        .cloned()
                        .collect(),
                );
            }
            return Some(
                three_of_a_kinds[0]
                    .iter()
                    .take(3)
                    .chain(pairs[0].iter())
                    .cloned()
                    .collect(),
            );
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::card::{Rank, Suit};

    use super::*;

    #[test]
    fn can_get_full_house() {
        let rank_map = hashmap! {
            Rank::Two => Card::from_cards_str("2d2c2h").unwrap(),
            Rank::Ace => Card::from_cards_str("AcAd").unwrap(),
        };
        assert_eq!(
            get_full_house(
                GameType::TexasHoldem,
                rank_map,
                &Card::from_cards_str("8h7c").unwrap()
            ),
            Some(vec![
                Card {
                    suit: Suit::Diamonds,
                    rank: Rank::Two
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Two
                },
                Card {
                    suit: Suit::Hearts,
                    rank: Rank::Two
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Ace
                },
                Card {
                    suit: Suit::Diamonds,
                    rank: Rank::Ace
                }
            ])
        );
    }

    #[test]
    fn can_get_full_house_omaha() {
        let rank_map = hashmap! {
            Rank::Two => Card::from_cards_str("2d2c2h").unwrap(),
            Rank::Ace => Card::from_cards_str("AcAd").unwrap(),
        };
        assert_eq!(
            get_full_house(
                GameType::Omaha,
                rank_map,
                &Card::from_cards_str("AcAd4d5d").unwrap()
            ),
            Some(vec![
                Card {
                    suit: Suit::Diamonds,
                    rank: Rank::Two
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Two
                },
                Card {
                    suit: Suit::Hearts,
                    rank: Rank::Two
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Ace
                },
                Card {
                    suit: Suit::Diamonds,
                    rank: Rank::Ace
                }
            ])
        );
    }

    #[test]
    fn cannot_get_full_house_omaha() {
        let rank_map = hashmap! {
            Rank::Two => Card::from_cards_str("2d2c2h").unwrap(),
            Rank::Ace => Card::from_cards_str("AcAd").unwrap(),
        };
        assert_eq!(
            get_full_house(
                GameType::Omaha,
                rank_map,
                &Card::from_cards_str("Ad4d5d").unwrap()
            ),
            None
        );
    }

    #[test]
    fn cannot_get_full_bad_cards() {
        let rank_map = hashmap! {
            Rank::Two => Card::from_cards_str("2c2h").unwrap(),
            Rank::Three => Card::from_cards_str("3d").unwrap(),
            Rank::Ace => Card::from_cards_str("AcAd").unwrap(),
        };
        assert_eq!(
            get_full_house(
                GameType::TexasHoldem,
                rank_map,
                &Card::from_cards_str("8h7c").unwrap()
            ),
            None
        );
    }
}
