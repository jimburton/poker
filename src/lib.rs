pub mod poker;

extern crate num_derive;
extern crate num_traits;

#[cfg(test)]
mod tests {
    use crate::poker::hand::{best_hand, group_by_rank, longest_sequence};
    use crate::poker::types::{Card, Hand, Rank, Suit};

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
            println!("group_by_rank(FOUR_OF_A_KIND) first group: {:?}", c);
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
                "best_hand(ONE_PAIR): expected Rank2, result was {:?}",
                r
            );
        } else {
            panic!(
                "best_hand(ONE_PAIR): expected Hand::OnePair, result was {:?}",
                bh_one_pair
            );
        }
    }
}
