use std::collections::HashMap;

use crate::{card::Rank, Card, GameType};

pub fn get_two_pairs(
    game_type: GameType,
    cards: &[Card],
    rank_map: &HashMap<Rank, Vec<Card>>,
    player_cards: &[Card],
    board: &[Card],
) -> Option<Vec<Card>> {
    let mut pairs: Vec<&Vec<Card>> = rank_map.values().filter(|cards| cards.len() == 2).collect();

    if pairs.len() < 2 {
        return None;
    }
    pairs.sort_by(|a, b| b[0].rank.partial_cmp(&a[0].rank).unwrap());

    match game_type {
        GameType::Omaha => {
            let top_pair = pairs.remove(0);

            for pair in pairs.into_iter() {
                let this_combo: Vec<&Card> = top_pair.iter().chain(pair.iter()).collect();
                let matches: Vec<&Card> = player_cards
                    .iter()
                    .filter(|pc| this_combo.iter().any(|c| **c == **pc))
                    .collect();

                if matches.len() == 2 {
                    let mut board: Vec<Card> = board.to_vec();
                    board.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());

                    return Some(
                        this_combo
                            .into_iter()
                            .chain(
                                board
                                    .iter()
                                    .filter(|pc| cards.iter().all(|c| *c != **pc))
                                    .take(1),
                            )
                            .cloned()
                            .collect(),
                    );
                } else if matches.len() == 1 {
                    let mut player_cards: Vec<Card> = player_cards.to_vec();
                    player_cards.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());

                    return Some(
                        this_combo
                            .into_iter()
                            .chain(
                                player_cards
                                    .iter()
                                    .filter(|pc| cards.iter().all(|c| *c != **pc))
                                    .take(1),
                            )
                            .cloned()
                            .collect(),
                    );
                }
            }

            None
        }
        _ => {
            return Some(
                pairs[0]
                    .iter()
                    .chain(pairs[1].iter())
                    .chain(
                        cards
                            .iter()
                            .filter(|c| c.rank != pairs[0][0].rank && c.rank != pairs[1][0].rank)
                            .take(1),
                    )
                    .cloned()
                    .collect(),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::card::{Rank, Suit};

    use super::*;

    #[test]
    fn can_get_two_pairs() {
        let rank_map = hashmap! {
            Rank::Two => Card::from_cards_str("2h2c").unwrap(),
            Rank::Ace => Card::from_cards_str("AcAd").unwrap(),
        };
        assert_eq!(
            get_two_pairs(
                GameType::TexasHoldem,
                &Card::from_cards_str("AcAd6d5c2h2c").unwrap(),
                &rank_map,
                &Card::from_cards_str("AcAd").unwrap(),
                &Card::from_cards_str("6d3c2h").unwrap()
            ),
            Some(vec![
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Ace
                },
                Card {
                    suit: Suit::Diamonds,
                    rank: Rank::Ace
                },
                Card {
                    suit: Suit::Hearts,
                    rank: Rank::Two
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Two
                },
                Card {
                    suit: Suit::Diamonds,
                    rank: Rank::Six
                },
            ])
        );
    }
}
