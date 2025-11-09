/// Functions for generating and organising sequences of cards.
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

use crate::poker::card::{Card, Rank, Suit};

/// Find the longest continuous sequence in a collection of cards.
pub fn longest_sequence(cards: &[Card]) -> Vec<Card> {
    if cards.is_empty() {
        return Vec::new();
    }

    // Extract unique ranks and sort them by their value.
    let unique_ranks_set: HashSet<Rank> = cards.iter().map(|card| card.rank).collect();
    let mut sorted_unique_ranks: Vec<Rank> = unique_ranks_set.into_iter().collect();
    sorted_unique_ranks.sort();

    if sorted_unique_ranks.is_empty() {
        return Vec::new();
    }

    // --- Find the range (start rank and length) of the longest sequence ---

    let mut max_length = 0;
    let mut best_start_value: u8 = 0; // Value of the starting rank (e.g., 2 for Rank2)

    let mut current_length = 1;
    let mut current_start_value = sorted_unique_ranks[0].value(); // Start with the first rank

    // Iterate to find the longest continuous sequence of unique ranks.
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

    // Compare the last sequence with the recorded max length.
    if current_length > max_length {
        max_length = current_length;
        best_start_value = current_start_value;
    }

    // Handle the case where the longest sequence is just a single rank.
    if max_length == 0 {
        max_length = 1;
        best_start_value = sorted_unique_ranks[0].value();
    }

    // Filter the original hand to collect the cards in the longest sequence (one card per rank) ---

    // Collect the actual cards, ensuring only one card is selected for each rank in the sequence.
    let mut final_sequence_cards: Vec<Card> = Vec::new();
    // Use a HashSet to track which ranks have already been added to the final result
    let mut included_ranks: HashSet<Rank> = HashSet::new();

    let min_rank_value = best_start_value;
    // The exclusive upper bound for the rank value
    let max_rank_value = best_start_value + max_length as u8;

    // Iterate through the original cards to find a single representative for each rank in the sequence.
    for card in cards.iter() {
        let rank_val = card.rank.value();

        // Check if the rank is within the longest sequence range.
        if rank_val >= min_rank_value && rank_val < max_rank_value {
            // 5b. Check if we have already included a card of this rank using HashSet::insert.
            if included_ranks.insert(card.rank) {
                // If insertion is successful (returns true), the rank is new for the result set.
                final_sequence_cards.push(*card);
            }
        }
    }

    // Sort the final sequence by rank for a clean, ordered result.
    final_sequence_cards.sort_by_key(|card| card.rank);

    final_sequence_cards
}

/// Group a collection of cards by their rank.
pub fn group_by_rank(cards: &[Card]) -> Vec<Vec<Card>> {
    let mut grouped_by_rank: HashMap<Rank, Vec<Card>> = HashMap::new();

    for card in cards.iter() {
        grouped_by_rank
            .entry(card.rank)
            // if the key doesn't exist, insert a new vec
            .or_default()
            // push the current card
            .push(*card);
    }
    let mut cs: Vec<Vec<Card>> = grouped_by_rank.into_values().collect();
    cs.sort_by_key(|b| Reverse(b.len()));
    cs
}

/// Group a collection of cards by their suit.
pub fn group_by_suit(cards: &[Card]) -> Vec<Vec<Card>> {
    let mut grouped_by_suit: HashMap<Suit, Vec<Card>> = HashMap::new();

    for card in cards.iter() {
        grouped_by_suit
            .entry(card.suit)
            // if the key doesn't exist, insert a new vec
            .or_default()
            // push the current card
            .push(*card);
    }
    let mut cs: Vec<Vec<Card>> = grouped_by_suit.into_values().collect();
    // sort the inner lists by rank descending
    cs.iter_mut()
        .for_each(|inner| inner.sort_by(|a, b| b.rank.cmp(&a.rank)));
    // sort the outer lists by length
    cs.sort_by_key(|b| Reverse(b.len()));
    cs
}

/// Predicate for a collection of cards being of the same suit.
pub fn same_suit(cards: &[Card]) -> bool {
    if cards.is_empty() {
        true
    } else {
        let c1 = cards[0];
        cards.iter().all(|a| a.suit == c1.suit)
    }
}

/// Tests for the sequence module.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::poker::card::{Card, Rank, Suit};
    use crate::poker::test_data::*;

    #[test]
    fn test_longest_sequence() {
        let h1 = Vec::from(ONE_PAIR_HC8);
        let ls_h1 = longest_sequence(&h1);
        let ls_h1_len = ls_h1.len();
        assert!(
            ls_h1.len() == 3,
            "Longest sequence: expected 3, result was {ls_h1_len}"
        );
        let h2 = Vec::from(FOUR_OF_A_KIND);
        let ls_h2 = longest_sequence(&h2);
        let ls_h2_len = ls_h2.len();
        assert!(
            ls_h2.len() == 1,
            "Longest sequence: expected 1, result was {ls_h2_len}"
        );
    }

    #[test]
    fn test_group_by_rank() {
        let h1 = Vec::from(ONE_PAIR_HC8);
        let gr_h1 = group_by_rank(&h1);
        assert!(
            gr_h1.len() == 4,
            "group_by_rank(ONE_PAIR).len(): expected 4 groups, result was {}",
            gr_h1.len()
        );
        if let Some(c) = gr_h1.first() {
            assert!(
                c.len() == 2,
                "group_by_rank(ONE_PAIR): longest group should be have 2 cards, was {}",
                c.len()
            );
            assert!(
                c.get(0).unwrap().rank == Rank::Rank2,
                "group_by_rank(ONE_PAIR): longest group should have Rank2 cards, was {:?}",
                c.get(0).unwrap().rank
            );
        } else {
            panic!("group_by_rank(ONE_PAIR): Nothing in the longest group")
        }
        let h2 = Vec::from(FOUR_OF_A_KIND);
        let gr_h2 = group_by_rank(&h2);
        assert!(
            gr_h2.len() == 2,
            "group_by_rank(FOUR_OF_A_KIND).len(): expected 2 groups, result was {}",
            gr_h2.len()
        );
        if let Some(c) = gr_h2.first() {
            assert!(
                c.len() == 4,
                "group_by_rank(FOUR_OF_A_KIND): longest group should be have 4 cards, was {}",
                c.len()
            );
            assert!(
                c.first().unwrap().rank == Rank::Rank5,
                "group_by_rank(FOUR_OF_A_KIND): longest group should have Rank5 cards, was {:?}",
                c.first().unwrap().rank
            );
        } else {
            panic!("group_by_rank(FOUR_OF_A_KIND): Nothing in the longest group")
        }
    }

    #[test]
    fn test_same_suit() {
        let h1: [Card; 3] = [
            Card {
                rank: Rank::Rank2,
                suit: Suit::Clubs,
            },
            Card {
                rank: Rank::Rank3,
                suit: Suit::Clubs,
            },
            Card {
                rank: Rank::Ace,
                suit: Suit::Clubs,
            },
        ];
        let good = same_suit(&Vec::from(h1));
        assert!(good, "same_suit(h1): expected true, was {}", good);
        let h2: [Card; 3] = [
            Card {
                rank: Rank::Rank2,
                suit: Suit::Clubs,
            },
            Card {
                rank: Rank::Rank3,
                suit: Suit::Clubs,
            },
            Card {
                rank: Rank::Ace,
                suit: Suit::Hearts,
            },
        ];
        let bad = same_suit(&Vec::from(h2));
        assert!(!bad, "same_suit(h2): expected false, was {}", bad);
    }
}
