use crate::{Card, GameType};

pub fn get_straight(game_type: GameType, cards: &[Card]) -> Option<Vec<Card>> {
    let mut card_match: Vec<&Card> = vec![];
    let mut card_match_omaha: Vec<Vec<&Card>> = vec![];
    for (i, card) in cards.iter().enumerate() {
        if i == 0 {
            card_match = vec![card];
            card_match_omaha = vec![vec![card]];
            continue;
        }

        let prev_card = *card_match.last().unwrap();

        if card.rank == prev_card.rank {
            card_match_omaha.iter_mut().last().unwrap().push(card);
        } else if card.rank as u8 + 1 == prev_card.rank as u8 {
            card_match.push(card);
            card_match_omaha.push(vec![card]);
        } else if card_match.len() < 5 {
            card_match = vec![card];
            card_match_omaha = vec![vec![card]];
            continue;
        }

        if card_match.len() >= 5 {
            match game_type {
                GameType::Omaha => {
                    // TODO add logic
                    return None;
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
        assert_eq!(
            get_straight(
                GameType::TexasHoldem,
                &Card::from_cards_str("8h7c6c5c4d").unwrap()
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
    fn can_get_straight_flush_bad_cards() {
        assert_eq!(
            get_straight(
                GameType::TexasHoldem,
                &Card::from_cards_str("Ac7c6c5c4c").unwrap()
            ),
            None
        );
    }
}
