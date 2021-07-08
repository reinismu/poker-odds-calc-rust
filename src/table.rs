use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

use crate::{
    game::{self, HandCombination},
    player::Player,
    Card, Cards, GameType,
};

#[derive(Debug, Clone)]
pub struct Table {
    players: Vec<Player>,
    community_cards: Vec<Card>,
    dead_cards: Vec<Card>,
}

#[derive(Debug)]
pub struct PlayerResult {
    pub hand: Vec<Card>,
    pub wins: u64,
    pub ties: u64,
    pub ranks: HashMap<HandCombination, u64>,
}

#[derive(Debug)]
pub struct Result {
    player_results: Vec<PlayerResult>,
    iterations: u64,
    approximate: bool,
}

impl Table {
    pub fn new(
        player_hands: Vec<Cards>,
        community_cards: Vec<Card>,
        dead_cards: Vec<Card>,
    ) -> Table {
        Table {
            players: player_hands
                .into_iter()
                .map(|cards| Player::new(cards.cards))
                .collect(),
            community_cards,
            dead_cards,
        }
    }

    fn get_available_cards(&self, game_type: GameType) -> Vec<Card> {
        match game_type {
            GameType::ShortdeckHoldem => Card::get_short_deck_cards(),
            _ => Card::get_all_cards(),
        }
        .into_iter()
        .filter(|c| {
            !self.dead_cards.iter().any(|dc| *dc == *c)
                && !self.community_cards.iter().any(|dc| *dc == *c)
                && !self
                    .players
                    .iter()
                    .any(|p| p.hand.iter().any(|hc| *hc == *c))
        })
        .collect()
    }

    pub fn get_results(
        &self,
        game_type: GameType,
        trips_beat_straight: bool,
        limit: u64,
    ) -> Result {
        let mut player_results: Vec<PlayerResult> = self
            .players
            .iter()
            .map(|p| PlayerResult {
                hand: p.hand.clone(),
                ties: 0,
                wins: 0,
                ranks: HashMap::new(),
            })
            .collect();

        let mut available_cards = self.get_available_cards(game_type);
        let mut iterations = 0u64;

        let mut add_results = |players: &[Player], board: &[Card]| {
            let hand_strengths = game::get_results(game_type, trips_beat_straight, players, board);

            let top_points = hand_strengths
                .iter()
                .max_by(|a, b| a.points.cmp(&b.points))
                .unwrap()
                .points;

            let is_tie = hand_strengths
                .iter()
                .filter(|hs| hs.points == top_points)
                .take(2)
                .count()
                > 1;

            for (i, pr) in player_results.iter_mut().enumerate() {
                let hand_strength = &hand_strengths[i];
                let rank = pr.ranks.entry(hand_strength.hand_combination).or_default();
                *rank += 1;

                if hand_strength.points == top_points {
                    if is_tie {
                        pr.ties += 1;
                    } else {
                        pr.wins += 1;
                    }
                }
            }
            iterations += 1;
        };

        let mut rng = thread_rng();
        for _ in 0..limit {
            let (shuffled_cards, _) = available_cards.partial_shuffle(&mut rng, 5);
            add_results(&self.players, shuffled_cards);
        }

        Result {
            player_results,
            iterations,
            approximate: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_result() {
        let table = Table::new(
            vec![
                Cards {
                    cards: Card::from_cards_str("AcAd").unwrap(),
                },
                Cards {
                    cards: Card::from_cards_str("AcAd").unwrap(),
                },
            ],
            Card::from_cards_str("As6s5s4s3s").unwrap(),
            vec![],
        );
        println!(
            "{:#?}",
            table.get_results(GameType::TexasHoldem, false, 10000)
        )
    }
}
