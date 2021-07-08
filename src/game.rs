use std::collections::HashMap;

use crate::card::{Card, Rank, Suit};

fn sort_by_rank_desc(cards: &mut Vec<Card>) {
    cards.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());
}

fn get_hand_strength(board: &[Card], player_cards: &[Card]) {
    let mut cards: Vec<Card> = board.iter().chain(player_cards).cloned().collect();
    sort_by_rank_desc(&mut cards);

    let mut suit_map: HashMap<Suit, Vec<Card>> = HashMap::new();
    let mut rank_map: HashMap<Rank, Vec<Card>> = HashMap::new();

    cards.iter().for_each(|c| {
        let entry = suit_map.entry(c.suit).or_insert_with(Vec::new);
        entry.push(*c);
        sort_by_rank_desc(entry);
        let entry = rank_map.entry(c.rank).or_insert_with(Vec::new);
        entry.push(*c);
        sort_by_rank_desc(entry);
    })
}
