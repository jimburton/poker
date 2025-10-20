use std::cmp::Ordering;

use crate::poker::card::{Card, Hand};
use crate::poker::game::Winner;
use crate::poker::player::PlayerHand;
use crate::poker::sequence::{group_by_rank, group_by_suit, longest_sequence, same_suit};

pub fn best_hand(cards: &Vec<Card>) -> Hand {
    let mut cs = cards.clone();
    cs.sort_by(|a, b| b.rank.cmp(&a.rank));
    let ls = longest_sequence(&cs);
    let ranks = group_by_rank(cards);
    let suits = group_by_suit(cards);
    if same_suit(cards) && ls.len() == 5 {
        Hand::StraightFlush(cards[cards.len() - 1].rank)
    } else if !ranks.is_empty() && ranks[0].len() == 4 {
        Hand::FourOfAKind(ranks[0][0].rank)
    } else if ranks.len() > 1 && ranks[0].len() == 3 && ranks[1].len() == 2 {
        Hand::FullHouse(ranks[0][0].rank, ranks[1][0].rank)
    } else if !suits.is_empty() && suits[0].len() >= 5 {
        let ls = suits.get(0).unwrap();
        Hand::Flush(ls[4].rank, ls[3].rank, ls[2].rank, ls[1].rank, ls[0].rank)
    } else if ls.len() >= 5 {
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
    hand_a: (String, Hand, Vec<Card>),
    hand_b: (String, Hand, Vec<Card>),
) -> Winner {
    // Placeholder logic for comparison: returns winner based on hand variant order
    let (name_a, h_a, c_a) = hand_a;
    let (name_b, h_b, c_b) = hand_b;

    if h_a > h_b {
        Winner::Winner {
            name: name_a,
            hand: h_a,
            cards: c_a,
        }
    } else if h_b > h_a {
        Winner::Winner {
            name: name_b,
            hand: h_b,
            cards: c_b,
        }
    } else {
        match (h_a, h_b) {
            // If two straight flushes have the same highest card, it's a draw
            (Hand::StraightFlush(_r1), Hand::StraightFlush(_r2)) => Winner::Draw(vec![
                PlayerHand {
                    name: name_a,
                    best_hand: h_a,
                    cards: c_a,
                },
                PlayerHand {
                    name: name_b,
                    best_hand: h_b,
                    cards: c_b,
                },
            ]),
            // No draw for two four of a kinds
            (Hand::FourOfAKind(r1), Hand::FourOfAKind(r2)) => {
                if r1 > r2 {
                    Winner::Winner {
                        name: name_a,
                        hand: h_a,
                        cards: c_a,
                    }
                } else {
                    Winner::Winner {
                        name: name_b,
                        hand: h_b,
                        cards: c_b,
                    }
                }
            }
            // For two full houses, the highest three of a kind wins, or if they are
            // the same rank, the highest pair wins. If they are the same it's a draw.
            (Hand::FullHouse(r1, r3), Hand::FullHouse(r2, r4)) => match r1.cmp(&r2) {
                Ordering::Greater => Winner::Winner {
                    name: name_a,
                    hand: h_a,
                    cards: c_a,
                },
                Ordering::Less => Winner::Winner {
                    name: name_b,
                    hand: h_b,
                    cards: c_b,
                },
                Ordering::Equal => match r3.cmp(&r4) {
                    Ordering::Greater => Winner::Winner {
                        name: name_a,
                        hand: h_a,
                        cards: c_a,
                    },
                    Ordering::Less => Winner::Winner {
                        name: name_b,
                        hand: h_b,
                        cards: c_b,
                    },
                    Ordering::Equal => Winner::Draw(vec![
                        PlayerHand {
                            name: name_a,
                            best_hand: h_a,
                            cards: c_a,
                        },
                        PlayerHand {
                            name: name_b,
                            best_hand: h_b,
                            cards: c_b,
                        },
                    ]),
                },
            },
            // if two players have a flush, the highest card wins. If the highest cards
            // are the same, the second highest wins, and so on, down to the fifth card.
            // If all five cards are the same, it's a draw.
            (Hand::Flush(..), Hand::Flush(..)) => {
                highest_cards((name_a, h_a, c_a), (name_b, h_b, c_b))
            }
            // if two players have a straight, the highest card wins. If the highest cards
            // are the same, the second highest wins, and so on, down to the fifth card.
            // If all five cards are the same, it's a draw.
            (Hand::Straight(_r1), Hand::Straight(_r2)) => {
                highest_cards((name_a, h_a, c_a), (name_b, h_b, c_b))
            }
            // if two players have three of a kind, the highest ranked triple wins. If they are the same, the highest
            // kicker wins (highest from the remaining two cards
            (Hand::ThreeOfAKind(_r1), Hand::ThreeOfAKind(_r2)) => {
                highest_cards((name_a, h_a, c_a), (name_b, h_b, c_b))
            }
            // if two players have two pair, the highest rank pair wins. If they are the same, the higher ranked
            // of the second pair wins. Otherwise the highest kicker wins.
            (Hand::TwoPair(_r1, _r3), Hand::TwoPair(_r2, _r4)) => {
                highest_cards((name_a, h_a, c_a), (name_b, h_b, c_b))
            }
            // if two players have one pair, the highest rank pair wins. Otherwise, decide by the kickers
            (Hand::OnePair(_r1), Hand::OnePair(_r2)) => {
                highest_cards((name_a, h_a, c_a), (name_b, h_b, c_b))
            }
            // If two players have the same high cards, compare the kickers
            (Hand::HighCard(_r1), Hand::HighCard(_r2)) => {
                highest_cards((name_a, h_a, c_a), (name_b, h_b, c_b))
            }
            _ => panic!("Not going to happen."),
        }
    }
}

fn highest_cards(hand_a: (String, Hand, Vec<Card>), hand_b: (String, Hand, Vec<Card>)) -> Winner {
    let (name_a, h_a, mut c_a) = hand_a;
    let (name_b, h_b, mut c_b) = hand_b;

    // Sort cards in descending order (highest rank first).
    // Since Card implements Ord, it sorts by Rank then Suit.
    c_a.sort_unstable_by(|a, b| b.cmp(a));
    c_b.sort_unstable_by(|a, b| b.cmp(a));
    // Compare card by card.
    for (card_a, card_b) in c_a.iter().zip(c_b.iter()) {
        if card_a.rank > card_b.rank {
            return Winner::Winner {
                name: name_a,
                hand: h_a,
                cards: c_a,
            };
        } else if card_b.rank > card_a.rank {
            return Winner::Winner {
                name: name_b,
                hand: h_b,
                cards: c_b,
            };
        }
    }

    // If the loop completes, all cards are identical (full draw).
    Winner::Draw(vec![
        PlayerHand {
            name: name_a,
            best_hand: h_a,
            cards: c_a,
        },
        PlayerHand {
            name: name_b,
            best_hand: h_b,
            cards: c_b,
        },
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::poker::card::{Hand, Rank};
    use crate::poker::test_data::*;

    #[test]
    fn test_highest_cards() {
        let p1 = "player1";
        let c1 = Vec::from(HIGH_CARD_TEN);
        let h1 = Hand::HighCard(Rank::Rank10);
        let p2 = "player2";
        let c2 = c1.clone();
        let w = highest_cards((p1.to_string(), h1, c1), (p2.to_string(), h1, c2));
        match w {
            Winner::Draw(winners) => {
                assert!(
                    winners.len() == 2,
                    "Expected two winners, got {}",
                    winners.len()
                );
            }
            Winner::Winner { name, .. } => panic!("Expected a draw but {} won.", name),
        }
        let c1 = Vec::from(HIGH_CARD_TEN);
        let p3 = "player3";
        let c3 = Vec::from(HIGH_CARD_ACE);
        let h3 = Hand::HighCard(Rank::Ace);
        let w = highest_cards((p1.to_string(), h1, c1), (p3.to_string(), h3, c3));
        match w {
            Winner::Draw(_winners) => {
                panic!("Expected a win for p3, draw");
            }
            Winner::Winner { name, .. } => {
                assert!(name == p3, "Expected p3, was {}.", name)
            }
        }
    }

    #[test]
    fn test_compare_hands() {
        let c1 = Vec::from(ONE_PAIR_8_1);
        let p1 = "player1";
        let h1 = Hand::OnePair(Rank::Rank8);
        let c2 = Vec::from(ONE_PAIR_8_2);
        let p2 = "player2";
        let h2 = Hand::OnePair(Rank::Rank8);
        let w = compare_hands((p1.to_string(), h1, c1), (p2.to_string(), h2, c2));
        match w {
            Winner::Draw(winners) => {
                assert!(
                    winners.len() == 2,
                    "Expected two winners, got {}",
                    winners.len()
                );
            }
            Winner::Winner { name, .. } => panic!("Expected a draw but {} won.", name),
        }
    }

    #[test]
    fn test_best_hand_high_card() {
        let h1 = Vec::from(HIGH_CARD_ACE);
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
        let h1 = Vec::from(ONE_PAIR_HC8);
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
            let mut ranks: [Rank; 2] = [r1, r2];
            ranks.sort();
            assert!(
                ranks[0] == Rank::Rank2 && ranks[1] == Rank::Rank4,
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
    fn test_best_hand_straight_seven_cards() {
        let h1 = Vec::from(STRAIGHT_7);
        let bh_s = best_hand(&h1);
        if let Hand::Straight(r) = bh_s {
            assert!(
                r == Rank::King,
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
}
