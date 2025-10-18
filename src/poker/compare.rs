use std::cmp::Ordering;
use std::iter::zip;

use crate::poker::sequence::{group_by_rank, longest_sequence, same_suit};
use crate::poker::types::{Card, Hand, Rank, Winner};

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
            (Hand::StraightFlush(r1), Hand::StraightFlush(r2)) => {
                Winner::Draw(vec![(name_a, h_a, c_a), (name_b, h_b, c_b)])
            }
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
                    Ordering::Equal => Winner::Draw(vec![(name_a, h_a, c_a), (name_b, h_b, c_b)]),
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
    Winner::Draw(vec![(name_a, h_a, c_a), (name_b, h_b, c_b)])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::poker::test_data::*;
    use crate::poker::types::{Card, Hand, Rank, Suit, Winner};

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
            Winner::Winner { name, hand, cards } => panic!("Expected a draw but {} won.", name),
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
            Winner::Winner { name, hand, cards } => {
                assert!(name == p3.clone(), "Expected p3, was {}.", name)
            }
        }
    }

    #[test]
    fn test_compare_hands() {
        let c1 = Vec::from(ONE_PAIR_8_1);
        let p1 = "player1";
        let h1 = Hand::OnePair(Rank::Rank5);
        let c2 = Vec::from(ONE_PAIR_8_2);
        let p2 = "player2";
        let h2 = Hand::OnePair(Rank::Rank5);
        let w = compare_hands((p1.to_string(), h1, c1), (p2.to_string(), h2, c2));
        match w {
            Winner::Draw(winners) => {
                assert!(
                    winners.len() == 2,
                    "Expected two winners, got {}",
                    winners.len()
                );
            }
            Winner::Winner { name, hand, cards } => panic!("Expected a draw but {} won.", name),
        }
    }
}
