use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;
use std::{collections::HashMap, sync::atomic::AtomicU64, time::Instant};

use crate::{
    game::{self, HandCombination, HandStrength},
    player::Player,
    Card, Cards, GameType,
};

#[derive(Debug, Clone)]
pub struct Table {
    players: Vec<Player>,
    community_cards: Vec<Card>,
    dead_cards: Vec<Card>,
}

#[derive(Debug, Clone)]
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

        let start_player_results: Vec<PlayerResult> = self
            .players
            .iter()
            .map(|p| PlayerResult {
                hand: p.hand.clone(),
                ties: 0,
                wins: 0,
                ranks: HashMap::new(),
            })
            .collect();

        let atomic_iterations = AtomicU64::new(0);

        let missing_card_count = 5 - self.community_cards.len();
        let mut unused_cards = self.get_unused_cards(game_type);

        // Shuffle for better approximation
        let mut rng = thread_rng();
        unused_cards.shuffle(&mut rng);

        let hand_strength_to_player_result =
            |results: Vec<PlayerResult>, hand_strengths: Vec<HandStrength>| -> Vec<PlayerResult> {
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

                results
                    .iter()
                    .enumerate()
                    .map(|(i, pr)| {
                        let mut new_pr = pr.clone();
                        let hand_strength = &hand_strengths[i];
                        let rank = new_pr
                            .ranks
                            .entry(hand_strength.hand_combination)
                            .or_default();
                        *rank += 1;

                        if hand_strength.points == top_points {
                            if is_tie {
                                new_pr.ties += 1;
                            } else {
                                new_pr.wins += 1;
                            }
                        }
                        new_pr
                    })
                    .collect()
            };

        let sum_player_results =
            |a: Vec<PlayerResult>, b: Vec<PlayerResult>| -> Vec<PlayerResult> {
                a.iter()
                    .zip(b)
                    .map(|(f, s)| {
                        let mut new_ranks = f.ranks.clone();

                        s.ranks.into_iter().for_each(|(k, v)| {
                            let rank = new_ranks.entry(k).or_default();
                            *rank += v;
                        });

                        PlayerResult {
                            hand: f.hand.clone(),
                            ties: f.ties + s.ties,
                            wins: f.wins + s.wins,
                            ranks: new_ranks,
                        }
                    })
                    .collect()
            };

        let player_results = unused_cards
            .iter()
            .combinations(missing_card_count)
            .take(limit as usize)
            .par_bridge()
            .map(|added_cards| {
                atomic_iterations.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                game::get_results(
                    game_type,
                    trips_beat_straight,
                    &self.players,
                    &self
                        .community_cards
                        .iter()
                        .chain(added_cards.into_iter())
                        .cloned()
                        .collect::<Vec<Card>>(),
                )
            })
            .fold(
                || start_player_results.clone(),
                hand_strength_to_player_result,
            )
            .reduce(|| start_player_results.clone(), sum_player_results);

        let iterations = atomic_iterations.load(std::sync::atomic::Ordering::SeqCst);
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
