pub use crate::poker::compare::{best_hand, compare_hands};
pub use crate::poker::sequence::{group_by_rank, longest_sequence, same_suit};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::poker::test_data::*;
    use crate::poker::types::{Card, Hand, Rank, Suit, Winner};

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

    #[test]
    fn test_compare_hands_simple() {
        let p1 = "player1";
        let p2 = "player2";
        let h_hca = Hand::HighCard(Rank::Ace);
        let h_op = Hand::OnePair(Rank::Rank8);
        let mut w = compare_hands(
            (p1.to_string(), h_hca, Vec::from(HIGH_CARD_ACE)),
            (p2.to_string(), h_op, Vec::from(ONE_PAIR_HC8)),
        );
        if let Winner::Winner {
            name,
            hand: _h,
            cards: _c,
        } = w
        {
            assert!(
                name == p2.to_string(),
                "compare_hands: expecting player1, was {:?}",
                name
            );
        } else {
            panic!("compare_hands: was expecting Winner, was {:?}", w);
        }
        let h_hca = Hand::HighCard(Rank::Ace);
        let h_hct = Hand::HighCard(Rank::Rank10);
        w = compare_hands(
            (p1.to_string(), h_hca, Vec::from(HIGH_CARD_ACE)),
            (p2.to_string(), h_hct, Vec::from(HIGH_CARD_TEN)),
        );
        if let Winner::Winner {
            name,
            hand: _h,
            cards: _c,
        } = w
        {
            assert!(
                name == p1.to_string(),
                "compare_hands: expecting player1, was {:?}",
                name
            );
        } else {
            panic!("compare_hands: was expecting Winner, was {:?}", w);
        }
        let hf_h = Hand::FullHouse(Rank::Rank2, Rank::Jack);
        w = compare_hands(
            (p1.to_string(), hf_h, Vec::from(FULL_HOUSE)),
            (p2.to_string(), hf_h, Vec::from(FULL_HOUSE)),
        );
        if let Winner::Draw(winners) = w {
            assert!(
                winners.len() == 2,
                "Expected two winners, was {}",
                winners.len()
            );
        } else {
            panic!("compare_hands: was expecting Draw, was {:?}", w);
        }
    }
}
