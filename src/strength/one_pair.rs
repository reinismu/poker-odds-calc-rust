use std::collections::HashMap;

use crate::{card::Rank, Card, GameType};

pub fn get_one_pair(
    game_type: GameType,
    cards: &[Card],
    rank_map: &HashMap<Rank, Vec<Card>>,
    player_cards: &[Card],
    board: &[Card],
) -> Option<Vec<Card>> {
    let pair = rank_map.values().find(|cards| cards.len() == 2);

    match pair {
        Some(pair) => match game_type {
            GameType::Omaha => {
                let matches: Vec<&Card> = player_cards
                    .iter()
                    .filter(|c| pair.iter().any(|pc| *pc == **c))
                    .collect();

                let mut board: Vec<Card> = board.to_vec();
                let mut player_cards: Vec<Card> = player_cards.to_vec();
                board.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());
                player_cards.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());

                let mut op: Vec<&Card> = vec![];

                if matches.len() == 2 {
                    op = board.iter().take(3).collect();
                } else if matches.len() == 1 {
                    op = board
                        .iter()
                        .filter(|c| !pair.iter().any(|pc| **c == *pc))
                        .take(2)
                        .chain(
                            player_cards
                                .iter()
                                .filter(|c| !pair.iter().any(|pc| **c == *pc))
                                .take(1),
                        )
                        .collect();
                } else if matches.is_empty() {
                    op = board
                        .iter()
                        .filter(|c| !pair.iter().any(|pc| **c == *pc))
                        .take(1)
                        .chain(player_cards.iter().take(2))
                        .collect();
                }

                if op.is_empty() {
                    None
                } else {
                    op.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());
                    Some(pair.iter().chain(op.into_iter()).cloned().collect())
                }
            }
            _ => {
                return Some(
                    pair.iter()
                        .chain(cards.iter().filter(|c| c.rank != pair[0].rank).take(3))
                        .cloned()
                        .collect(),
                )
            }
        },
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::card::{Rank, Suit};

    use super::*;

    #[test]
    fn can_get_one_pair() {
        let rank_map = hashmap! {
            Rank::Two => Card::from_cards_str("6d3c2h").unwrap(),
            Rank::Ace => Card::from_cards_str("AcAd").unwrap(),
        };
        assert_eq!(
            get_one_pair(
                GameType::TexasHoldem,
                &Card::from_cards_str("AcAd7d6d3c2h").unwrap(),
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
                    suit: Suit::Diamonds,
                    rank: Rank::Seven
                },
                Card {
                    suit: Suit::Diamonds,
                    rank: Rank::Six
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Three
                },
            ])
        );
    }
}
