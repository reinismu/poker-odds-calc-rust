use std::collections::HashMap;

use crate::{card::Rank, Card, GameType};

pub fn get_quads(
    game_type: GameType,
    cards: &[Card],
    rank_map: HashMap<Rank, Vec<Card>>,
    player_cards: &[Card],
    board: &[Card],
) -> Option<Vec<Card>> {
    let matches: Vec<&Vec<Card>> = rank_map.values().filter(|cards| cards.len() == 4).collect();

    if matches.is_empty() {
        return None;
    }

    match game_type {
        GameType::Omaha => {
            for quad_match in matches.into_iter() {
                let player_card_matches: Vec<&Card> = player_cards
                    .iter()
                    .filter(|c| quad_match.iter().any(|pc| *pc == **c))
                    .collect();

                if player_card_matches.len() == 2 {
                    let mut board: Vec<Card> = board.to_vec();
                    board.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());
                    let highest_board_kicker = board.iter().find(|c| c.rank != quad_match[0].rank);

                    match highest_board_kicker {
                        Some(highest_board_kicker) => {
                            return Some(
                                quad_match
                                    .iter()
                                    .chain([*highest_board_kicker].iter())
                                    .cloned()
                                    .collect(),
                            );
                        }
                        None => {
                            return Some(quad_match.to_vec());
                        }
                    }
                } else if player_card_matches.len() == 1 {
                    let mut player_cards: Vec<Card> = player_cards.to_vec();
                    player_cards.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());
                    let highest_board_kicker =
                        player_cards.iter().find(|c| c.rank != quad_match[0].rank);

                    match highest_board_kicker {
                        Some(highest_board_kicker) => {
                            return Some(
                                quad_match
                                    .iter()
                                    .chain([*highest_board_kicker].iter())
                                    .cloned()
                                    .collect(),
                            );
                        }
                        None => {
                            return Some(quad_match.to_vec());
                        }
                    }
                }
            }
            None
        }
        _ => {
            let highest_board_kicker = cards.iter().find(|c| c.rank != matches[0][0].rank);

            match highest_board_kicker {
                Some(highest_board_kicker) => {
                    return Some(
                        matches[0]
                            .iter()
                            .chain([*highest_board_kicker].iter())
                            .cloned()
                            .collect(),
                    );
                }
                None => Some(matches[0].to_vec()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::card::{Rank, Suit};

    use super::*;

    #[test]
    fn can_get_quads() {
        let rank_map = hashmap! {
            Rank::Two => Card::from_cards_str("2h2c2d2s").unwrap(),
            Rank::Ace => Card::from_cards_str("AcAd").unwrap(),
        };
        assert_eq!(
            get_quads(
                GameType::TexasHoldem,
                &Card::from_cards_str("AcAd2h2c2d2s").unwrap(),
                rank_map,
                &Card::from_cards_str("2h2c").unwrap(),
                &Card::from_cards_str("2d2sAcAd").unwrap()
            ),
            Some(vec![
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
                    rank: Rank::Two
                },
                Card {
                    suit: Suit::Spades,
                    rank: Rank::Two
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Ace
                }
            ])
        );
    }

    #[test]
    fn cannot_get_quads() {
        let rank_map = hashmap! {
            Rank::Two => Card::from_cards_str("6d3c2h").unwrap(),
            Rank::Ace => Card::from_cards_str("AcAd").unwrap(),
        };
        assert_eq!(
            get_quads(
                GameType::TexasHoldem,
                &Card::from_cards_str("AcAd6d3c2h").unwrap(),
                rank_map,
                &Card::from_cards_str("AcAd").unwrap(),
                &Card::from_cards_str("6d3c2h").unwrap()
            ),
            None
        );
    }
}
