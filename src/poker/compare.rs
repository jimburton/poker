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
