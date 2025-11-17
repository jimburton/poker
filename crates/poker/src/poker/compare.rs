/// Functions for comparing and ranking collections of cards.
use std::cmp::Ordering;

use crate::poker::{
    card::{BestHand, Card, Hand},
    player::{PlayerHand, Winner},
    sequence,
};

/// Get the best hand from a collection of cards.
pub fn best_hand(cards: &[Card]) -> BestHand {
    let mut cs = cards.to_owned();
    cs.sort_by(|a, b| b.rank.cmp(&a.rank));
    let longest_seq = sequence::longest_sequence(&cs);
    let ranks = sequence::group_by_rank(&cs);
    let suits = sequence::group_by_suit(&cs);
    if sequence::same_suit(cards) && longest_seq.len() == 5 {
        BestHand {
            hand: Hand::StraightFlush(cards[cards.len() - 1].rank),
            cards: cs,
        }
    } else if !ranks.is_empty() && ranks[0].len() == 4 {
        BestHand {
            hand: Hand::FourOfAKind(ranks[0][0].rank),
            cards: ranks[0].to_owned(),
        }
    } else if ranks.len() > 1 && ranks[0].len() == 3 && ranks[1].len() == 2 {
        let mut cards = ranks[0].clone();
        cards.append(&mut ranks[1].clone());
        BestHand {
            hand: Hand::FullHouse(ranks[0][0].rank, ranks[1][0].rank),
            cards,
        }
    } else if !suits.is_empty() && suits[0].len() >= 5 {
        let ls = suits.first().unwrap();
        BestHand {
            hand: Hand::Flush(ls[4].rank, ls[3].rank, ls[2].rank, ls[1].rank, ls[0].rank),
            cards: ls.to_owned(),
        }
    } else if longest_seq.len() >= 5 {
        BestHand {
            hand: Hand::Straight(cards.iter().map(|a| a.rank).max().unwrap()),
            cards: longest_seq,
        }
    } else if !ranks.is_empty() && ranks[0].len() == 3 {
        BestHand {
            hand: Hand::ThreeOfAKind(ranks[0][0].rank),
            cards: ranks[0].to_owned(),
        }
    } else if ranks.len() > 1 && ranks[0].len() == 2 && ranks[1].len() == 2 {
        let mut cards = ranks[0].clone();
        cards.append(&mut ranks[1].clone());
        cards.sort();
        BestHand {
            hand: Hand::TwoPair(ranks[0][0].rank, ranks[1][0].rank),
            cards,
        }
    } else if !ranks.is_empty() && ranks[0].len() == 2 {
        BestHand {
            hand: Hand::OnePair(ranks[0][0].rank),
            cards: ranks[0].to_owned(),
        }
    } else if let Some(c) = cards.iter().max() {
        BestHand {
            hand: Hand::HighCard(c.rank),
            cards: vec![c.to_owned()],
        }
    } else {
        panic!("Called best hand with empty set of cards.");
    }
}

/// Compare two hands, resulting in a winner or a draw.
pub fn compare_hands(hand_a: PlayerHand, hand_b: PlayerHand) -> Winner {
    // Placeholder logic for comparison: returns winner based on hand variant order
    let (name_a, h_a, c_a) = (hand_a.name, hand_a.hand, hand_a.cards);
    let (name_b, h_b, c_b) = (hand_b.name, hand_b.hand, hand_b.cards);

    if h_a.hand > h_b.hand {
        Winner::SoleWinner(PlayerHand {
            name: name_a,
            hand: h_a,
            cards: c_a,
        })
    } else if h_b.hand > h_a.hand {
        Winner::SoleWinner(PlayerHand {
            name: name_b,
            hand: h_b,
            cards: c_b,
        })
    } else {
        match (h_a.hand, h_b.hand) {
            // If two straight flushes have the same highest card, it's a draw
            (Hand::StraightFlush(_r1), Hand::StraightFlush(_r2)) => Winner::Draw(vec![
                PlayerHand {
                    name: name_a,
                    hand: h_a,
                    cards: c_a,
                },
                PlayerHand {
                    name: name_b,
                    hand: h_b,
                    cards: c_b,
                },
            ]),
            // No draw for two 4oK
            (Hand::FourOfAKind(r1), Hand::FourOfAKind(r2)) => {
                if r1 > r2 {
                    Winner::SoleWinner(PlayerHand {
                        name: name_a,
                        hand: h_a,
                        cards: c_a,
                    })
                } else {
                    Winner::SoleWinner(PlayerHand {
                        name: name_b,
                        hand: h_b,
                        cards: c_b,
                    })
                }
            }
            // For two full houses the highest 3oK wins, or if they are
            // the same rank, the highest pair wins. If the pairs are the same it's a draw.
            (Hand::FullHouse(r1, r3), Hand::FullHouse(r2, r4)) => match r1.cmp(&r2) {
                Ordering::Greater => Winner::SoleWinner(PlayerHand {
                    name: name_a,
                    hand: h_a,
                    cards: c_a,
                }),
                Ordering::Less => Winner::SoleWinner(PlayerHand {
                    name: name_b,
                    hand: h_b,
                    cards: c_b,
                }),
                Ordering::Equal => match r3.cmp(&r4) {
                    Ordering::Greater => Winner::SoleWinner(PlayerHand {
                        name: name_a,
                        hand: h_a,
                        cards: c_a,
                    }),
                    Ordering::Less => Winner::SoleWinner(PlayerHand {
                        name: name_b,
                        hand: h_b,
                        cards: c_b,
                    }),
                    Ordering::Equal => Winner::Draw(vec![
                        PlayerHand {
                            name: name_a,
                            hand: h_a,
                            cards: c_a,
                        },
                        PlayerHand {
                            name: name_b,
                            hand: h_b,
                            cards: c_b,
                        },
                    ]),
                },
            },
            // if the players each have one of the other types of hand then
            // their cards are compared pairwise. If all five cards are the same, it's a draw.
            (Hand::Flush(..), Hand::Flush(..))
            | (Hand::Straight(..), Hand::Straight(..))
            | (Hand::ThreeOfAKind(..), Hand::ThreeOfAKind(..))
            | (Hand::TwoPair(..), Hand::TwoPair(..))
            | (Hand::OnePair(..), Hand::OnePair(..))
            | (Hand::HighCard(..), Hand::HighCard(..)) => highest_cards(
                PlayerHand {
                    name: name_a,
                    hand: h_a,
                    cards: c_a,
                },
                PlayerHand {
                    name: name_b,
                    hand: h_b,
                    cards: c_b,
                },
            ),
            _ => panic!("Not going to happen."),
        }
    }
}

/// Decide which hand has the highest cards by comparing them pair-wise.
fn highest_cards(hand_a: PlayerHand, hand_b: PlayerHand) -> Winner {
    let (name_a, h_a, mut c_a) = (hand_a.name, hand_a.hand, hand_a.cards);
    let (name_b, h_b, mut c_b) = (hand_b.name, hand_b.hand, hand_b.cards);

    // Sort cards in descending order (highest rank first).
    // Since Card implements Ord, it sorts by Rank then Suit.
    c_a.sort_unstable_by(|a, b| b.cmp(a));
    c_b.sort_unstable_by(|a, b| b.cmp(a));
    // Compare card by card.
    for (card_a, card_b) in c_a.iter().zip(c_b.iter()) {
        if card_a.rank > card_b.rank {
            return Winner::SoleWinner(PlayerHand {
                name: name_a,
                hand: h_a,
                cards: c_a,
            });
        } else if card_b.rank > card_a.rank {
            return Winner::SoleWinner(PlayerHand {
                name: name_b,
                hand: h_b,
                cards: c_b,
            });
        }
    }

    // If the loop completes, all cards are identical (full draw).
    Winner::Draw(vec![
        PlayerHand {
            name: name_a,
            hand: h_a,
            cards: c_a,
        },
        PlayerHand {
            name: name_b,
            hand: h_b,
            cards: c_b,
        },
    ])
}

/// Tests for the compare module.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::poker::card::{Hand, Rank, Suit};
    use crate::poker::test_data::*;

    #[test]
    fn test_highest_cards() {
        let p1 = "player1";
        let c1 = Vec::from(HIGH_CARD_TEN);
        let h1 = Hand::HighCard(Rank::Rank10);
        let p2 = "player2";
        let c2 = c1.clone();
        let w = highest_cards(
            PlayerHand {
                name: p1.to_string(),
                hand: BestHand {
                    hand: h1,
                    cards: c1.clone(),
                },
                cards: c1,
            },
            PlayerHand {
                name: p2.to_string(),
                hand: BestHand {
                    hand: h1,
                    cards: c2.clone(),
                },
                cards: c2,
            },
        );
        match w {
            Winner::Draw(winners) => {
                assert!(
                    winners.len() == 2,
                    "Expected two winners, got {}",
                    winners.len()
                );
            }
            Winner::SoleWinner(PlayerHand { name, .. }) => {
                panic!("Expected a draw but {} won.", name)
            }
        }
        let c1 = Vec::from(HIGH_CARD_TEN);
        let p3 = "player3";
        let c3 = Vec::from(HIGH_CARD_ACE);
        let h3 = Hand::HighCard(Rank::Ace);
        let w = highest_cards(
            PlayerHand {
                name: p1.to_string(),
                hand: BestHand {
                    hand: h1,
                    cards: c1.clone(),
                },
                cards: c1,
            },
            PlayerHand {
                name: p3.to_string(),
                hand: BestHand {
                    hand: h3,
                    cards: c3.clone(),
                },
                cards: c3,
            },
        );
        match w {
            Winner::Draw(_winners) => {
                panic!("Expected a win for p3, draw");
            }
            Winner::SoleWinner(PlayerHand { name, .. }) => {
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
        let w = compare_hands(
            PlayerHand {
                name: p1.to_string(),
                hand: BestHand {
                    hand: h1,
                    cards: c1.clone(),
                },
                cards: c1,
            },
            PlayerHand {
                name: p2.to_string(),
                hand: BestHand {
                    hand: h2,
                    cards: c2.clone(),
                },
                cards: c2,
            },
        );
        match w {
            Winner::Draw(winners) => {
                assert!(
                    winners.len() == 2,
                    "Expected two winners, got {}",
                    winners.len()
                );
            }
            Winner::SoleWinner(PlayerHand { name, .. }) => {
                panic!("Expected a draw but {} won.", name)
            }
        }
    }

    #[test]
    fn test_best_hand_high_card() {
        let h1 = Vec::from(HIGH_CARD_ACE);
        let bh_high_card = best_hand(&h1);
        assert!(
            bh_high_card.cards.len() == 1,
            "Expected one card in best_hand.cards, was {:?}",
            bh_high_card.cards
        );
        let high_card = bh_high_card.cards[0];
        assert!(
            high_card.rank == Rank::Ace && high_card.suit == Suit::Spades,
            "Expected Ace of Spades as best_hand.cards, was {:?}",
            high_card
        );
        if let Hand::HighCard(r) = bh_high_card.hand {
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
        assert!(
            bh_one_pair.cards.len() == 2,
            "Expected two cards in best_hand.cards, was {:?}",
            bh_one_pair.cards
        );
        let card1 = bh_one_pair.cards[0];
        let card2 = bh_one_pair.cards[1];
        assert!(
            card1.rank == Rank::Rank2 && card2.rank == Rank::Rank2,
            "Expected a pair of twos in best_hand.cards, was {:?}",
            bh_one_pair.cards
        );
        if let Hand::OnePair(r) = bh_one_pair.hand {
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
        assert!(
            bh_two_pair.cards.len() == 4,
            "Expected four cards in best_hand.cards, was {:?}",
            bh_two_pair.cards
        );
        let card1 = bh_two_pair.cards[0];
        let card2 = bh_two_pair.cards[1];
        let card3 = bh_two_pair.cards[2];
        let card4 = bh_two_pair.cards[3];
        assert!(
            card1.rank == Rank::Rank2
                && card2.rank == Rank::Rank2
                && card3.rank == Rank::Rank4
                && card4.rank == Rank::Rank4,
            "Expected pairs of twos and fours in best_hand.cards, was {:?}",
            bh_two_pair.cards
        );
        if let Hand::TwoPair(r1, r2) = bh_two_pair.hand {
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
        assert!(
            bh_tok.cards.len() == 3,
            "Expected three cards in best_hand.cards, was {:?}",
            bh_tok.cards
        );
        let card1 = bh_tok.cards[0];
        let card2 = bh_tok.cards[1];
        let card3 = bh_tok.cards[2];
        assert!(
            card1.rank == Rank::Rank3 && card2.rank == Rank::Rank3 && card3.rank == Rank::Rank3,
            "Expected three threes in best_hand.cards, was {:?}",
            bh_tok.cards
        );
        if let Hand::ThreeOfAKind(r) = bh_tok.hand {
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
        assert!(
            bh_s.cards.len() == 5,
            "Expected fivecards in best_hand.cards, was {:?}",
            bh_s.cards
        );
        let card1 = bh_s.cards[0];
        let card2 = bh_s.cards[1];
        let card3 = bh_s.cards[2];
        let card4 = bh_s.cards[3];
        let card5 = bh_s.cards[4];
        assert!(
            card1.rank == Rank::Rank2
                && card2.rank == Rank::Rank3
                && card3.rank == Rank::Rank4
                && card4.rank == Rank::Rank5
                && card5.rank == Rank::Rank6,
            "Expected Straight from Rank2 to Rank6 in best_hand.cards, was {:?}",
            bh_s.cards
        );
        if let Hand::Straight(r) = bh_s.hand {
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
        assert!(
            bh_s.cards.len() == 5,
            "Expected fivecards in best_hand.cards, was {:?}",
            bh_s.cards
        );
        let card1 = bh_s.cards[0];
        let card2 = bh_s.cards[1];
        let card3 = bh_s.cards[2];
        let card4 = bh_s.cards[3];
        let card5 = bh_s.cards[4];
        assert!(
            card1.rank == Rank::Rank4
                && card2.rank == Rank::Rank5
                && card3.rank == Rank::Rank6
                && card4.rank == Rank::Rank7
                && card5.rank == Rank::Rank8,
            "Expected Straight from Rank4 to Rank8 in best_hand.cards, was {:?}",
            bh_s.cards
        );
        if let Hand::Straight(r) = bh_s.hand {
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
        assert!(
            bh_f.cards.len() == 5,
            "Expected five cards in best_hand.cards, was {:?}",
            bh_f.cards
        );
        if let Hand::Flush(r1, r2, r3, r4, r5) = bh_f.hand {
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
        assert!(
            bh_f.cards.len() == 5,
            "Expected five cards in best_hand.cards, was {:?}",
            bh_f.cards
        );
        if let Hand::FullHouse(r1, r2) = bh_f.hand {
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
        assert!(
            bh_f.cards.len() == 4,
            "Expected four cards in best_hand.cards, was {:?}",
            bh_f.cards
        );
        if let Hand::FourOfAKind(r) = bh_f.hand {
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
        assert!(
            bh_sf.cards.len() == 5,
            "Expected five cards in best_hand.cards, was {:?}",
            bh_sf.cards
        );
        if let Hand::StraightFlush(r) = bh_sf.hand {
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
