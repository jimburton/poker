use crate::poker::types::{Card, Hand, Rank, Winner};
use std::collections::{HashMap, HashSet};
use std::iter::zip;

fn longest_sequence(cards: &Vec<Card>) -> Vec<Card> {
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

fn group_by_rank(cards: &Vec<Card>) -> Vec<Vec<Card>> {
    let mut grouped_by_rank: HashMap<Rank, Vec<Card>> = HashMap::new();

    for card in cards.iter() {
        grouped_by_rank
            .entry(card.rank)
            // if the key doesn't exist, insert a new vec
            .or_insert_with(Vec::new)
            // push the current card
            .push(*card);
    }
    grouped_by_rank.into_values().collect()
}

fn same_suit(cards: &Vec<Card>) -> bool {
    if cards.len() == 0 {
        true
    } else {
        let c1 = cards[0];
        cards.iter().all(|a| a.suit == c1.suit)
    }
}

pub fn best_hand(cards: &Vec<Card>) -> Hand {
    let mut cs = cards.clone();
    cs.sort_by(|a, b| b.rank.cmp(&a.rank));
    let ls = longest_sequence(&cs);
    let ranks = group_by_rank(cards);
    if same_suit(cards) && ls.len() == 5 {
        Hand::StraightFlush(cards[cards.len() - 1].rank)
    } else if ranks.len() > 0 && ranks[0].len() == 4 {
        Hand::FourOfAKind(ranks[0][0].rank)
    } else if ranks.len() > 1 && ranks[0].len() == 3 && ranks[1].len() == 2 {
        Hand::FullHouse(ranks[0][0].rank, ranks[1][0].rank)
    } else if same_suit(cards) {
        Hand::Flush(
            cards[0].rank,
            cards[1].rank,
            cards[2].rank,
            cards[3].rank,
            cards[4].rank,
        )
    } else if ls.len() == 5 {
        Hand::Straight(cards.iter().map(|a| a.rank).max().unwrap())
    } else if ranks.len() > 0 && ranks[0].len() == 3 {
        Hand::ThreeOfAKind(ranks[0][0].rank)
    } else if ranks.len() > 1 && ranks[0].len() == 2 && ranks[1].len() == 2 {
        Hand::TwoPair(ranks[0][0].rank, ranks[1][0].rank)
    } else if ranks.len() > 0 && ranks[0].len() == 2 {
        Hand::OnePair(ranks[0][0].rank)
    } else {
        Hand::HighCard(cards.iter().max().unwrap().rank)
    }
}

fn compare_hands(
    (name1, cs1): (&String, &Vec<Card>),
    (name2, cs2): (&String, &Vec<Card>),
) -> Winner {
    let h1 = best_hand(cs1);
    let h2 = best_hand(cs2);
    match h2.cmp(&h1) {
        GT => Winner::Winner {
            name: *name1,
            hand: h1,
        },
        LT => Winner::Winner {
            name: *name2,
            hand: h2,
        },
        EQ => match (h1, h2) {
            (Hand::StraightFlush(r1), Hand::StraightFlush(r2)) => win_or_draw(r1, r2),
            (Hand::FourOfAKind(r1), Hand::FourOfAKind(r2)) => win_or_high(r1, r2),
            (Hand::FullHouse(r1, r3), Hand::FullHouse(r2, r4)) => win_or_high2(r1, r2, r3, r4),
            (Hand::Flush(..), Hand::Flush(..)) => highest_cards((name1, cs1), (name2, cs2), h1, h2),
            (Hand::Straight(r1), Hand::Straight(r2)) => win_or_draw(r1, r2),
            (Hand::ThreeOfAKind(r1), Hand::ThreeOfAKind(r2)) => win_or_high(r1, r2),
            (Hand::TwoPair(r1, r3), Hand::TwoPair(r2, r4)) => win_or_high2(r1, r2, r3, r4),
            (Hand::OnePair(r1), Hand::OnePair(r2)) => win_or_high(r1, r2),
            (Hand::HighCard(r1), Hand::HighCard(r2)) => win_or_high(r1, r2),
            _ => panic!("Not going to happen."),
        },
    }
}

fn win_or_draw(r1: &Rank, h1: Hand, name1: String, r2: &Rank, h2: Hand, name2: String) -> Winner {
    match r1.cmp(r2) {
        _EQ => Winner::Draw,
        _LT => Winner::Winner {
            name: name2,
            hand: h2,
        },
        _GT => Winner::Winner {
            name: name1,
            hand: h1,
        },
    }
}

fn win_or_high(
    r1: &Rank,
    h1: Hand,
    name1: String,
    cs1: &Vec<Card>,
    r2: &Rank,
    h2: Hand,
    name2: String,
    cs2: &Vec<Card>,
) -> Winner {
    match r1.cmp(r2) {
        _EQ => highest_cards((name1, cs1), (name2, cs2), h1, h2),
        _LT => Winner::Winner {
            name: name2,
            hand: h2,
        },
        _GT => Winner::Winner {
            name: name1,
            hand: h1,
        },
    }
}

fn win_or_high2(
    r1: &Rank,
    r2: &Rank,
    r3: &Rank,
    r4: &Rank,
    h1: Hand,
    name1: String,
    cs1: &Vec<Card>,
    h2: Hand,
    name2: String,
    cs2: &Vec<Card>,
) -> Winner {
    match r1.cmp(r2) {
        _EQ => match r3.cmp(r4) {
            EQ => highest_cards((name1, cs1), (name2, cs2), h1, h2),
            LT => Winner::Winner {
                name: name1,
                hand: h1,
            },
            GT => Winner::Winner {
                name: name2,
                hand: h2,
            },
        },
        LT => Winner::Winner {
            name: name2,
            hand: h2,
        },
        GT => Winner::Winner {
            name: name1,
            hand: h1,
        },
    }
}

fn highest_cards(
    (name1, cs1): (String, &Vec<Card>),
    (name2, cs2): (String, &Vec<Card>),
    h1: Hand,
    h2: Hand,
) -> Winner {
    let mut c1 = cs1.clone();
    c1.sort_by(|a, b| b.cmp(a));
    let mut c2 = cs2.clone();
    c2.sort_by(|a, b| b.cmp(a));
    let iter = zip(c1, c2);
    let mut result = Winner::Draw;
    for (d1, d2) in iter {
        if d1 > d2 {
            result = Winner::Winner {
                name: name1,
                hand: h1,
            };
            break;
        } else if d2 > d1 {
            result = Winner::Winner {
                name: name2,
                hand: h2,
            };
            break;
        }
    }
    result
}
