use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{collections::HashMap, time::Instant};

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
    pub player_results: Vec<PlayerResult>,
    pub iterations: u64,
    pub approximate: bool,
    pub time_in_ms: u64,
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

    fn get_unused_cards(&self, game_type: GameType) -> Vec<Card> {
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
        limit: u64,
        trips_beat_straight: bool,
    ) -> Result {
        let start_instant = Instant::now();

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
        let missing_card_count = 5 - self.community_cards.len();
        let mut unused_cards = self.get_unused_cards(game_type);

        // Shuffle for better approximation
        let mut rng = thread_rng();
        unused_cards.shuffle(&mut rng);

        for added_cards in unused_cards
            .iter()
            .combinations(missing_card_count)
            .take(limit as usize)
        {
            add_results(
                &self.players,
                &self
                    .community_cards
                    .iter()
                    .chain(added_cards.into_iter())
                    .cloned()
                    .collect::<Vec<Card>>(),
            );
        }

        Result {
            player_results,
            iterations,
            approximate: iterations >= limit,
            time_in_ms: start_instant.elapsed().as_millis() as u64,
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
            table.get_results(GameType::TexasHoldem, 10000, false)
        )
    }

    #[test]
    fn can_get_correct_result() {
        let table = Table::new(
            vec![
                Cards {
                    cards: Card::from_cards_str("AdKc").unwrap(),
                },
                Cards {
                    cards: Card::from_cards_str("Ac7c").unwrap(),
                },
            ],
            Card::from_cards_str("2s3s4s5s").unwrap(),
            vec![],
        );
        let result = table.get_results(GameType::TexasHoldem, 10000, false);
        assert_eq!(
            result.player_results[0]
                .ranks
                .get(&HandCombination::Straight),
            Some(&35u64)
        );
    }

    #[test]
    fn can_get_correct_result_2() {
        let table = Table::new(
            vec![
                Cards {
                    cards: Card::from_cards_str("AdKc").unwrap(),
                },
                Cards {
                    cards: Card::from_cards_str("Ac7c").unwrap(),
                },
            ],
            Card::from_cards_str("2s3s4s5s6c").unwrap(),
            vec![],
        );
        let result = table.get_results(GameType::TexasHoldem, 10000, false);
        assert_eq!(result.player_results[1].wins, 1);
    }
}
