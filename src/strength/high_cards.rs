use crate::{Card, GameType};

pub fn get_high_cards(
    game_type: GameType,
    cards: &[Card],
    player_cards: &[Card],
    board: &[Card],
) -> Vec<Card> {
    match game_type {
        GameType::Omaha => {
            let mut board: Vec<Card> = board.to_vec();
            let mut player_cards: Vec<Card> = player_cards.to_vec();
            board.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());
            player_cards.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());
            let mut cards: Vec<Card> = player_cards
                .into_iter()
                .take(2)
                .chain(board.into_iter().take(3))
                .collect();

            cards.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());

            cards
        }
        _ => return cards.iter().take(5).cloned().collect(),
    }
}

#[cfg(test)]
mod tests {
    use crate::card::{Rank, Suit};

    use super::*;

    #[test]
    fn can_get_high_cards() {
        assert_eq!(
            get_high_cards(
                GameType::TexasHoldem,
                &Card::from_cards_str("8h7c6c5c4d4c2c").unwrap(),
                &Card::from_cards_str("8h7c").unwrap(),
                &Card::from_cards_str("4d4c2c").unwrap()
            ),
            vec![
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
            ]
        );
    }
    #[test]
    fn can_get_high_cards_omaha() {
        assert_eq!(
            get_high_cards(
                GameType::Omaha,
                &Card::from_cards_str("8h7c6c5c4d4c2c").unwrap(),
                &Card::from_cards_str("8h7c6c").unwrap(),
                &Card::from_cards_str("4d4c2c").unwrap()
            ),
            vec![
                Card {
                    suit: Suit::Hearts,
                    rank: Rank::Eight
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Seven
                },
                Card {
                    suit: Suit::Diamonds,
                    rank: Rank::Four
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Four
                },
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Two
                }
            ]
        );
    }
}
