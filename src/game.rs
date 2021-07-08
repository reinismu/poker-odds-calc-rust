use std::collections::HashMap;

use crate::{
    card::{Card, Rank},
    player::Player,
    strength::{
        flush::get_flush, full_house::get_full_house, high_cards::get_high_cards,
        one_pair::get_one_pair, quads::get_quads, straight::get_straight,
        straight_flush::get_straight_flush, three_of_a_kind::get_three_of_a_kind,
        two_pairs::get_two_pairs,
    },
    GameType,
};

fn sort_by_rank_desc(cards: &mut Vec<Card>) {
    cards.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum HandCombination {
    RoyalFlush,
    StraightFlush,
    Quads,
    FullHouse,
    Flush,
    Straight,
    TreeOfAKind,
    TwoPairs,
    OnePair,
    HighCards,
}

pub struct HandStrength {
    pub hand_combination: HandCombination,
    pub points: u64,
}

pub fn get_results(
    game_type: GameType,
    trips_beat_straight: bool,
    players: &[Player],
    board: &[Card],
) -> Vec<HandStrength> {
    players
        .iter()
        .map(|p| get_hand_strength(game_type, board, &p.hand, trips_beat_straight))
        .collect()
}

fn calc_points(start_points: u64, cards: &[Card]) -> u64 {
    let mut points = start_points;
    cards.iter().for_each(|c| {
        points *= 14;
        points += c.rank as u64;
    });
    points
}

fn get_hand_strength(
    game_type: GameType,
    board: &[Card],
    player_cards: &[Card],
    trips_beat_straight: bool,
) -> HandStrength {
    let mut cards: Vec<Card> = board.iter().chain(player_cards).cloned().collect();
    sort_by_rank_desc(&mut cards);

    let mut suit_map = HashMap::with_capacity(4);
    let mut rank_map = HashMap::with_capacity(12);

    for c in cards.iter() {
        let entry = suit_map.entry(c.suit).or_insert_with(Vec::new);
        entry.push(*c);
        sort_by_rank_desc(entry);
        let entry = rank_map.entry(c.rank).or_insert_with(Vec::new);
        entry.push(*c);
        sort_by_rank_desc(entry);
    }

    if let Some(hand) = get_straight_flush(game_type, &suit_map, &player_cards) {
        if hand[0].rank == Rank::Ace {
            return HandStrength {
                hand_combination: HandCombination::RoyalFlush,
                points: calc_points(10, &hand),
            };
        }
        return HandStrength {
            hand_combination: HandCombination::StraightFlush,
            points: calc_points(9, &hand),
        };
    }

    if let Some(hand) = get_quads(game_type, &cards, &rank_map, &player_cards, &board) {
        return HandStrength {
            hand_combination: HandCombination::Quads,
            points: calc_points(8, &hand),
        };
    }

    if game_type != GameType::ShortdeckHoldem {
        if let Some(hand) = get_full_house(game_type, &rank_map, &player_cards) {
            return HandStrength {
                hand_combination: HandCombination::FullHouse,
                points: calc_points(7, &hand),
            };
        }
        if let Some(hand) = get_flush(game_type, &suit_map, &player_cards, &board) {
            return HandStrength {
                hand_combination: HandCombination::Flush,
                points: calc_points(6, &hand),
            };
        }
        if let Some(hand) = get_straight(game_type, &cards) {
            return HandStrength {
                hand_combination: HandCombination::Straight,
                points: calc_points(5, &hand),
            };
        }
        if let Some(hand) = get_three_of_a_kind(game_type, &cards, &rank_map, &player_cards, &board)
        {
            return HandStrength {
                hand_combination: HandCombination::TreeOfAKind,
                points: calc_points(4, &hand),
            };
        }
    } else {
        if let Some(hand) = get_flush(game_type, &suit_map, &player_cards, &board) {
            return HandStrength {
                hand_combination: HandCombination::Flush,
                points: calc_points(7, &hand),
            };
        }
        if let Some(hand) = get_full_house(game_type, &rank_map, &player_cards) {
            return HandStrength {
                hand_combination: HandCombination::FullHouse,
                points: calc_points(6, &hand),
            };
        }
        if trips_beat_straight {
            if let Some(hand) =
                get_three_of_a_kind(game_type, &cards, &rank_map, &player_cards, &board)
            {
                return HandStrength {
                    hand_combination: HandCombination::TreeOfAKind,
                    points: calc_points(5, &hand),
                };
            }
            if let Some(hand) = get_straight(game_type, &cards) {
                return HandStrength {
                    hand_combination: HandCombination::Straight,
                    points: calc_points(4, &hand),
                };
            }
        } else {
            if let Some(hand) = get_straight(game_type, &cards) {
                return HandStrength {
                    hand_combination: HandCombination::Straight,
                    points: calc_points(5, &hand),
                };
            }
            if let Some(hand) =
                get_three_of_a_kind(game_type, &cards, &rank_map, &player_cards, &board)
            {
                return HandStrength {
                    hand_combination: HandCombination::TreeOfAKind,
                    points: calc_points(4, &hand),
                };
            }
        }
    }

    if let Some(hand) = get_two_pairs(game_type, &cards, &rank_map, &player_cards, &board) {
        return HandStrength {
            hand_combination: HandCombination::TwoPairs,
            points: calc_points(3, &hand),
        };
    }

    if let Some(hand) = get_one_pair(game_type, &cards, &rank_map, &player_cards, &board) {
        return HandStrength {
            hand_combination: HandCombination::OnePair,
            points: calc_points(2, &hand),
        };
    }

    HandStrength {
        hand_combination: HandCombination::HighCards,
        points: calc_points(1, &get_high_cards(game_type, &cards, &player_cards, &board)),
    }
}
