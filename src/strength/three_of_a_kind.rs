use std::collections::HashMap;

use crate::{card::Rank, Card, GameType};

pub fn get_three_of_a_kind(
    game_type: GameType,
    cards: &[Card],
    rank_map: &HashMap<Rank, Vec<Card>>,
    player_cards: &[Card],
    board: &[Card],
) -> Option<Vec<Card>> {
    let matches: Vec<&Vec<Card>> = rank_map.values().filter(|cards| cards.len() == 3).collect();

    if matches.is_empty() {
        return None;
    }

    match game_type {
        GameType::Omaha => {
            let mut board: Vec<Card> = board.to_vec();
            board.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());
            let mut player_cards: Vec<Card> = player_cards.to_vec();
            player_cards.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());

            for three_match in matches.into_iter() {
                let player_cards_used: Vec<&Card> = player_cards
                    .iter()
                    .filter(|c| three_match.iter().any(|pc| *pc == **c))
                    .collect();
                let mut op: Vec<&Card> = vec![];

                if player_cards_used.len() > 2 {
                    op = board
                        .iter()
                        .filter(|c| three_match.iter().take(3).all(|pc| *pc != **c))
                        .take(2)
                        .collect();
                } else if player_cards_used.len() == 2 {
                    op = board
                        .iter()
                        .filter(|c| three_match.iter().all(|pc| *pc != **c))
                        .take(2)
                        .collect();
                } else if player_cards_used.len() == 1 {
                    op = board
                        .iter()
                        .filter(|c| three_match.iter().all(|pc| *pc != **c))
                        .chain(
                            player_cards
                                .iter()
                                .filter(|c| three_match.iter().all(|pc| *pc != **c)),
                        )
                        .collect();
                } else if player_cards_used.is_empty() {
                    op = player_cards.iter().take(2).collect();
                }

                if !op.is_empty() {
                    return Some(three_match.iter().chain(op.into_iter()).cloned().collect());
                }
            }
            None
        }
        _ => {
            return Some(
                matches[0]
                    .iter()
                    .chain(
                        cards
                            .iter()
                            .filter(|c| c.rank != matches[0][0].rank)
                            .take(2),
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
    fn can_get_three_of_a_kind() {
        let rank_map = hashmap! {
            Rank::Two => Card::from_cards_str("2h2c2d").unwrap(),
            Rank::Ace => Card::from_cards_str("AcAd").unwrap(),
        };
        assert_eq!(
            get_three_of_a_kind(
                GameType::TexasHoldem,
                &Card::from_cards_str("AcAd3h2h2c2d").unwrap(),
                &rank_map,
                &Card::from_cards_str("2h2c").unwrap(),
                &Card::from_cards_str("AcAd3h2d2s").unwrap()
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
    fn cannot_get_three_of_a_kind() {
        let rank_map = hashmap! {
            Rank::Two => Card::from_cards_str("2d2c").unwrap(),
            Rank::Ace => Card::from_cards_str("AcAd").unwrap(),
        };
        assert_eq!(
            get_three_of_a_kind(
                GameType::TexasHoldem,
                &Card::from_cards_str("AcAd7d2d2c").unwrap(),
                &rank_map,
                &Card::from_cards_str("AcAd").unwrap(),
                &Card::from_cards_str("7d2d2c").unwrap()
            ),
            None
        );
    }
}
