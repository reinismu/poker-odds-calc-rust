use std::collections::HashMap;

use crate::{card::Suit, Card, GameType};

pub fn get_flush(
    game_type: GameType,
    suit_map: &HashMap<Suit, Vec<Card>>,
    player_cards: &Vec<Card>,
    board: &Vec<Card>,
) -> Option<Vec<Card>> {
    for (_, cards) in suit_map.iter() {
        if cards.len() >= 5 {
            if game_type != GameType::Omaha {
                return Some(cards.iter().cloned().take(5).collect());
            }
            let mut player_cards_in_play: Vec<&Card> = player_cards
                .iter()
                .filter(|pc| cards.iter().any(|c| *c == **pc))
                .collect();
            if player_cards_in_play.len() < 2 {
                continue;
            }
            let mut board_cards_in_play: Vec<&Card> = board
                .iter()
                .filter(|pc| cards.iter().any(|c| *c == **pc))
                .collect();
            if board_cards_in_play.len() < 3 {
                continue;
            }
            player_cards_in_play.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());
            board_cards_in_play.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());

            let mut flush: Vec<&Card> = player_cards_in_play
                .into_iter()
                .take(2)
                .chain(board_cards_in_play.into_iter().take(3))
                .collect();
            flush.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());

            return Some(flush.into_iter().cloned().collect());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::card::Rank;

    use super::*;

    #[test]
    fn can_get_flush() {
        let suit_map = hashmap! {
            Suit::Hearts => vec![],
            Suit::Clubs => Card::from_cards_str("9c7c6c5c4c").unwrap(),
        };
        assert_eq!(
            get_flush(GameType::TexasHoldem, &suit_map, &vec![], &vec![]),
            Some(vec![
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Nine
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
                    suit: Suit::Clubs,
                    rank: Rank::Four
                }
            ])
        );
    }

    #[test]
    fn cannot_get_flush_bad_cards() {
        let suit_map = hashmap! {
            Suit::Hearts => Card::from_cards_str("2h").unwrap(),
            Suit::Clubs => Card::from_cards_str("8c7c6c5c").unwrap(),
        };
        assert_eq!(
            get_flush(GameType::TexasHoldem, &suit_map, &vec![], &vec![]),
            None
        );
    }
}
