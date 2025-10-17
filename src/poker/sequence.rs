use std::collections::{HashMap, HashSet};

use crate::poker::types::{Card, Rank};

pub fn longest_sequence(cards: &Vec<Card>) -> Vec<Card> {
    if cards.is_empty() {
        return Vec::new();
    }

    // 1. Extract all unique ranks and sort them by their value.
    let unique_ranks_set: HashSet<Rank> = cards.iter().map(|card| card.rank).collect();
    let mut sorted_unique_ranks: Vec<Rank> = unique_ranks_set.into_iter().collect();
    sorted_unique_ranks.sort();

    if sorted_unique_ranks.is_empty() {
        return Vec::new();
    }

    // --- Find the range (start rank and length) of the longest sequence ---

    // 2. Initialize tracking variables.
    let mut max_length = 0;
    let mut best_start_value: u8 = 0; // Value of the starting rank (e.g., 2 for Rank2)

    let mut current_length = 1;
    let mut current_start_value = sorted_unique_ranks[0].value(); // Start with the first rank

    // 3. Iterate to find the longest continuous sequence of unique ranks.
    for i in 1..sorted_unique_ranks.len() {
        let current_rank_val = sorted_unique_ranks[i].value();
        let previous_rank_val = sorted_unique_ranks[i - 1].value();

        if current_rank_val == previous_rank_val + 1 {
            // Sequence continues
            current_length += 1;
        } else {
            // Sequence breaks. Check if the current sequence is the new max.
            if current_length > max_length {
                max_length = current_length;
                best_start_value = current_start_value;
            }

            // Reset the current sequence tracker
            current_length = 1;
            current_start_value = current_rank_val;
        }
    }

    // 4. Final check: Compare the last sequence with the recorded max length.
    if current_length > max_length {
        max_length = current_length;
        best_start_value = current_start_value;
    }

    // Handle the case where the longest sequence is just a single rank.
    if max_length == 0 {
        max_length = 1;
        best_start_value = sorted_unique_ranks[0].value();
    }

    // --- Filter the original hand to collect the cards in the longest sequence (one card per rank) ---

    // 5. Collect the actual cards, ensuring only one card is selected for each rank in the sequence.
    let mut final_sequence_cards: Vec<Card> = Vec::new();
    // Use a HashSet to track which ranks have already been added to the final result
    let mut included_ranks: HashSet<Rank> = HashSet::new();

    let min_rank_value = best_start_value;
    // The exclusive upper bound for the rank value
    let max_rank_value = best_start_value + max_length as u8;

    // Iterate through the original cards to find a single representative for each rank in the sequence.
    for card in cards.iter() {
        let rank_val = card.rank.value();

        // 5a. Check if the rank is within the longest sequence range.
        if rank_val >= min_rank_value && rank_val < max_rank_value {
            // 5b. Check if we have already included a card of this rank using HashSet::insert.
            if included_ranks.insert(card.rank) {
                // If insertion is successful (returns true), the rank is new for the result set.
                final_sequence_cards.push(*card);
            }
        }
    }

    // 6. Sort the final sequence by rank for a clean, ordered result.
    final_sequence_cards.sort_by_key(|card| card.rank);

    final_sequence_cards
}

pub fn group_by_rank(cards: &Vec<Card>) -> Vec<Vec<Card>> {
    let mut grouped_by_rank: HashMap<Rank, Vec<Card>> = HashMap::new();

    for card in cards.iter() {
        grouped_by_rank
            .entry(card.rank)
            // if the key doesn't exist, insert a new vec
            .or_insert_with(Vec::new)
            // push the current card
            .push(*card);
    }
    let mut cs: Vec<Vec<Card>> = grouped_by_rank.into_values().collect();
    cs.sort_by(|a, b| b.len().cmp(&a.len()));
    cs
}

pub fn same_suit(cards: &Vec<Card>) -> bool {
    if cards.len() == 0 {
        true
    } else {
        let c1 = cards[0];
        cards.iter().all(|a| a.suit == c1.suit)
    }
}
