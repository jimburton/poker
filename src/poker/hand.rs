use crate::poker::types::{Card, Hand, Rank, Winner};
use std::cmp::Ordering;
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
    let mut cs: Vec<Vec<Card>> = grouped_by_rank.into_values().collect();
    cs.sort_by(|a, b| b.len().cmp(&a.len()));
    cs
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
    } else if !ranks.is_empty() && ranks[0].len() == 4 {
        Hand::FourOfAKind(ranks[0][0].rank)
    } else if ranks.len() > 1 && ranks[0].len() == 3 && ranks[1].len() == 2 {
        Hand::FullHouse(ranks[0][0].rank, ranks[1][0].rank)
    } else if same_suit(cards) {
        Hand::Flush(cs[4].rank, cs[3].rank, cs[2].rank, cs[1].rank, cs[0].rank)
    } else if ls.len() == 5 {
        Hand::Straight(cards.iter().map(|a| a.rank).max().unwrap())
    } else if ranks.len() > 0 && ranks[0].len() == 3 {
        Hand::ThreeOfAKind(ranks[0][0].rank)
    } else if ranks.len() > 1 && ranks[0].len() == 2 && ranks[1].len() == 2 {
        Hand::TwoPair(ranks[0][0].rank, ranks[1][0].rank)
    } else if !ranks.is_empty() && ranks[0].len() == 2 {
        Hand::OnePair(ranks[0][0].rank)
    } else {
        Hand::HighCard(cards.iter().max().unwrap().rank)
    }
}

pub fn compare_hands(
    (name1, cs1): (String, &Vec<Card>),
    (name2, cs2): (String, &Vec<Card>),
) -> Winner {
    let h1 = best_hand(cs1);
    let h2 = best_hand(cs2);
    match h1.cmp(&h2) {
        Ordering::Greater => Winner::Winner {
            name: name1,
            hand: h1,
        },
        Ordering::Less => Winner::Winner {
            name: name2,
            hand: h2,
        },
        Ordering::Equal => match (h1, h2) {
            (Hand::StraightFlush(r1), Hand::StraightFlush(r2)) => {
                win_or_draw(&r1, h1, name1, &r2, h2, name2)
            }
            (Hand::FourOfAKind(r1), Hand::FourOfAKind(r2)) => {
                win_or_high(&r1, h1, name1, cs1, &r2, h2, name2, cs2)
            }
            (Hand::FullHouse(r1, r3), Hand::FullHouse(r2, r4)) => {
                win_or_high2(&r1, &r2, &r3, &r4, h1, name1, cs1, h2, name2, cs2)
            }
            (Hand::Flush(..), Hand::Flush(..)) => highest_cards((name1, cs1), (name2, cs2), h1, h2),
            (Hand::Straight(r1), Hand::Straight(r2)) => win_or_draw(&r1, h1, name1, &r2, h2, name2),

            (Hand::ThreeOfAKind(r1), Hand::ThreeOfAKind(r2)) => {
                win_or_high(&r1, h1, name1, cs1, &r2, h2, name2, cs2)
            }
            (Hand::TwoPair(r1, r3), Hand::TwoPair(r2, r4)) => {
                win_or_high2(&r1, &r2, &r3, &r4, h1, name1, cs1, h2, name2, cs2)
            }
            (Hand::OnePair(r1), Hand::OnePair(r2)) => {
                win_or_high(&r1, h1, name1, cs1, &r2, h2, name2, cs2)
            }
            (Hand::HighCard(r1), Hand::HighCard(r2)) => {
                win_or_high(&r1, h1, name1, cs1, &r2, h2, name2, cs2)
            }
            _ => panic!("Not going to happen."),
        },
    }
}

fn win_or_draw(r1: &Rank, h1: Hand, name1: String, r2: &Rank, h2: Hand, name2: String) -> Winner {
    match r1.cmp(r2) {
        Ordering::Equal => Winner::Draw,
        Ordering::Less => Winner::Winner {
            name: name2,
            hand: h2,
        },
        Ordering::Greater => Winner::Winner {
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
        Ordering::Equal => highest_cards((name1, cs1), (name2, cs2), h1, h2),
        Ordering::Less => Winner::Winner {
            name: name2,
            hand: h2,
        },
        Ordering::Greater => Winner::Winner {
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
        Ordering::Equal => match r3.cmp(r4) {
            Ordering::Equal => highest_cards((name1, cs1), (name2, cs2), h1, h2),
            Ordering::Less => Winner::Winner {
                name: name1,
                hand: h1,
            },
            Ordering::Greater => Winner::Winner {
                name: name2,
                hand: h2,
            },
        },
        Ordering::Less => Winner::Winner {
            name: name2,
            hand: h2,
        },
        Ordering::Greater => Winner::Winner {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::poker::types::{Card, Rank, Suit};

    const HIGH_CARD: [Card; 5] = [
        Card {
            rank: Rank::Rank2,
            suit: Suit::Clubs,
        },
        Card {
            rank: Rank::Rank4,
            suit: Suit::Diamonds,
        },
        Card {
            rank: Rank::Rank7,
            suit: Suit::Clubs,
        },
        Card {
            rank: Rank::Rank10,
            suit: Suit::Hearts,
        },
        Card {
            rank: Rank::Ace,
            suit: Suit::Spades,
        },
    ];
    const ONE_PAIR: [Card; 5] = [
        Card {
            rank: Rank::Rank2,
            suit: Suit::Clubs,
        },
        Card {
            rank: Rank::Rank2,
            suit: Suit::Diamonds,
        },
        Card {
            rank: Rank::Rank3,
            suit: Suit::Clubs,
        },
        Card {
            rank: Rank::Rank4,
            suit: Suit::Hearts,
        },
        Card {
            rank: Rank::Rank8,
            suit: Suit::Spades,
        },
    ];
    const TWO_PAIR: [Card; 5] = [
        Card {
            rank: Rank::Rank2,
            suit: Suit::Clubs,
        },
        Card {
            rank: Rank::Rank2,
            suit: Suit::Diamonds,
        },
        Card {
            rank: Rank::Rank3,
            suit: Suit::Clubs,
        },
        Card {
            rank: Rank::Rank4,
            suit: Suit::Hearts,
        },
        Card {
            rank: Rank::Rank4,
            suit: Suit::Spades,
        },
    ];
    const THREE_OF_A_KIND: [Card; 5] = [
        Card {
            rank: Rank::Rank2,
            suit: Suit::Clubs,
        },
        Card {
            rank: Rank::King,
            suit: Suit::Diamonds,
        },
        Card {
            rank: Rank::Rank3,
            suit: Suit::Clubs,
        },
        Card {
            rank: Rank::Rank3,
            suit: Suit::Hearts,
        },
        Card {
            rank: Rank::Rank3,
            suit: Suit::Spades,
        },
    ];
    const STRAIGHT: [Card; 5] = [
        Card {
            rank: Rank::Rank2,
            suit: Suit::Clubs,
        },
        Card {
            rank: Rank::Rank3,
            suit: Suit::Diamonds,
        },
        Card {
            rank: Rank::Rank4,
            suit: Suit::Clubs,
        },
        Card {
            rank: Rank::Rank5,
            suit: Suit::Hearts,
        },
        Card {
            rank: Rank::Rank6,
            suit: Suit::Spades,
        },
    ];
    const FLUSH: [Card; 5] = [
        Card {
            rank: Rank::Rank2,
            suit: Suit::Clubs,
        },
        Card {
            rank: Rank::King,
            suit: Suit::Clubs,
        },
        Card {
            rank: Rank::Rank3,
            suit: Suit::Clubs,
        },
        Card {
            rank: Rank::Rank8,
            suit: Suit::Clubs,
        },
        Card {
            rank: Rank::Ace,
            suit: Suit::Clubs,
        },
    ];
    const FULL_HOUSE: [Card; 5] = [
        Card {
            rank: Rank::Rank2,
            suit: Suit::Clubs,
        },
        Card {
            rank: Rank::Rank2,
            suit: Suit::Diamonds,
        },
        Card {
            rank: Rank::Rank2,
            suit: Suit::Spades,
        },
        Card {
            rank: Rank::Jack,
            suit: Suit::Hearts,
        },
        Card {
            rank: Rank::Jack,
            suit: Suit::Spades,
        },
    ];
    const FOUR_OF_A_KIND: [Card; 5] = [
        Card {
            rank: Rank::Rank5,
            suit: Suit::Clubs,
        },
        Card {
            rank: Rank::Rank5,
            suit: Suit::Diamonds,
        },
        Card {
            rank: Rank::Rank5,
            suit: Suit::Spades,
        },
        Card {
            rank: Rank::Rank5,
            suit: Suit::Hearts,
        },
        Card {
            rank: Rank::Rank3,
            suit: Suit::Spades,
        },
    ];
    const STRAIGHT_FLUSH: [Card; 5] = [
        Card {
            rank: Rank::Rank5,
            suit: Suit::Clubs,
        },
        Card {
            rank: Rank::Rank6,
            suit: Suit::Clubs,
        },
        Card {
            rank: Rank::Rank7,
            suit: Suit::Clubs,
        },
        Card {
            rank: Rank::Rank8,
            suit: Suit::Clubs,
        },
        Card {
            rank: Rank::Rank9,
            suit: Suit::Clubs,
        },
    ];
    #[test]
    fn test_longest_sequence() {
        let h1 = Vec::from(ONE_PAIR);
        let ls_h1 = longest_sequence(&h1);
        let ls_h1_len = ls_h1.len();
        //println!("Longest sequence: {:?}", ls_h1);
        assert!(
            ls_h1.len() == 3,
            "Longest sequence: expected 3, result was {ls_h1_len}"
        );
        let h2 = Vec::from(HIGH_CARD);
        let ls_h2 = longest_sequence(&h2);
        let ls_h2_len = ls_h2.len();
        //println!("Longest sequence: {:?}", ls_h2);
        assert!(
            ls_h2.len() == 1,
            "Longest sequence: expected 1, result was {ls_h2_len}"
        );
    }

    #[test]
    fn test_group_by_rank() {
        let h1 = Vec::from(ONE_PAIR);
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
                c.get(0).unwrap().rank == Rank::Rank5,
                "group_by_rank(FOUR_OF_A_KIND): longest group should have Rank5 cards, was {:?}",
                c.get(0).unwrap().rank
            );
        } else {
            panic!("group_by_rank(FOUR_OF_A_KIND): Nothing in the longest group")
        }
    }

    #[test]
    fn test_best_hand_high_card() {
        let h1 = Vec::from(HIGH_CARD);
        let bh_high_card = best_hand(&h1);
        if let Hand::HighCard(r) = bh_high_card {
            assert!(
                r == Rank::Ace,
                "best_hand(HIGH_CARD): expected Ace, result was {:?}",
                r
            );
        } else {
            panic!(
                "best_hand(HIGH_CARD): expected Hand::HIGH_CARD, result was {:?}",
                bh_high_card
            );
        }
    }

    #[test]
    fn test_best_hand_one_pair() {
        let h1 = Vec::from(ONE_PAIR);
        let bh_one_pair = best_hand(&h1);
        if let Hand::OnePair(r) = bh_one_pair {
            assert!(
                r == Rank::Rank2,
                "best_hand(ONE_PAIR): expected 2, result was {:?}",
                r
            );
        } else {
            panic!(
                "best_hand(ONE_PAIR): expected Hand::OnePair, result was {:?}",
                bh_one_pair
            );
        }
    }

    #[test]
    fn test_best_hand_two_pair() {
        let h1 = Vec::from(TWO_PAIR);
        let bh_two_pair = best_hand(&h1);
        if let Hand::TwoPair(r1, r2) = bh_two_pair {
            assert!(
                r1 == Rank::Rank2 && r2 == Rank::Rank4,
                "best_hand(TWO_PAIR): expected 2, 4, result was {:?},{:?}",
                r1,
                r2
            );
        } else {
            panic!(
                "best_hand(ONE_PAIR): expected Hand::OnePair, result was {:?}",
                bh_two_pair
            );
        }
    }

    #[test]
    fn test_best_hand_three_of_a_kind() {
        let h1 = Vec::from(THREE_OF_A_KIND);
        let bh_tok = best_hand(&h1);
        if let Hand::ThreeOfAKind(r) = bh_tok {
            assert!(
                r == Rank::Rank3,
                "best_hand(THREE_OF_A_KIND): expected 3, result was {:?}",
                r
            );
        } else {
            panic!(
                "best_hand(THREE_OF_A_KIND): expected Hand::ThreeOfAKind, result was {:?}",
                bh_tok
            );
        }
    }

    #[test]
    fn test_best_hand_straight() {
        let h1 = Vec::from(STRAIGHT);
        let bh_s = best_hand(&h1);
        if let Hand::Straight(r) = bh_s {
            assert!(
                r == Rank::Rank6,
                "best_hand(STRAIGHT): expected 6, result was {:?}",
                r
            );
        } else {
            panic!(
                "best_hand(STRAIGHT): expected Hand::Straight, result was {:?}",
                bh_s
            );
        }
    }

    #[test]
    fn test_best_hand_flush() {
        let h1 = Vec::from(FLUSH);
        let bh_f = best_hand(&h1);
        if let Hand::Flush(r1, r2, r3, r4, r5) = bh_f {
            assert!(
                r1 == Rank::Rank2
                    && r2 == Rank::Rank3
                    && r3 == Rank::Rank8
                    && r4 == Rank::King
                    && r5 == Rank::Ace,
                "best_hand(FLUSH): expected 2,3,8,K,A, result was {:?},{:?},{:?},{:?},{:?}",
                r1,
                r2,
                r3,
                r4,
                r5
            );
        } else {
            panic!(
                "best_hand(FLUSH): expected Hand::Flush, result was {:?}",
                bh_f
            );
        }
    }

    #[test]
    fn test_best_full_house() {
        let h1 = Vec::from(FULL_HOUSE);
        let bh_f = best_hand(&h1);
        if let Hand::FullHouse(r1, r2) = bh_f {
            assert!(
                r1 == Rank::Rank2 && r2 == Rank::Jack,
                "best_hand(FULL_HOUSE): expected 2,J, result was {:?},{:?}",
                r1,
                r2
            );
        } else {
            panic!(
                "best_hand(FULL_HOUSE): expected Hand::FullHouse, result was {:?}",
                bh_f
            );
        }
    }

    #[test]
    fn test_best_four_of_a_kind() {
        let h1 = Vec::from(FOUR_OF_A_KIND);
        let bh_f = best_hand(&h1);
        if let Hand::FourOfAKind(r) = bh_f {
            assert!(
                r == Rank::Rank5,
                "best_hand(FOUR_OF_A_KIND): expected 5, result was {:?}",
                r
            );
        } else {
            panic!(
                "best_hand(FOUR_OF_A_KIND): expected Hand::FourOfAKind, result was {:?}",
                bh_f
            );
        }
    }

    #[test]
    fn test_best_straight_flush() {
        let h1 = Vec::from(STRAIGHT_FLUSH);
        let bh_sf = best_hand(&h1);
        if let Hand::StraightFlush(r) = bh_sf {
            assert!(
                r == Rank::Rank9,
                "best_hand(STRAIGHT_FLUSH): expected 9, result was {:?}",
                r
            );
        } else {
            panic!(
                "best_hand(STRAIGHT_FLUSH): expected Hand::StraightFlush, result was {:?}",
                bh_sf
            );
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
