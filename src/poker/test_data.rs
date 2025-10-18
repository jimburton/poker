use crate::poker::types::{Card, Rank, Suit};

pub const HIGH_CARD_TEN: [Card; 5] = [
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
        rank: Rank::Rank3,
        suit: Suit::Spades,
    },
];
pub const HIGH_CARD_ACE: [Card; 5] = [
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
pub const ONE_PAIR_8_1: [Card; 7] = [
    Card {
        rank: Rank::Rank5,
        suit: Suit::Spades,
    },
    Card {
        rank: Rank::Ace,
        suit: Suit::Hearts,
    },
    Card {
        rank: Rank::Rank8,
        suit: Suit::Spades,
    },
    Card {
        rank: Rank::Rank4,
        suit: Suit::Spades,
    },
    Card {
        rank: Rank::Rank3,
        suit: Suit::Diamonds,
    },
    Card {
        rank: Rank::Rank2,
        suit: Suit::Diamonds,
    },
    Card {
        rank: Rank::Rank8,
        suit: Suit::Diamonds,
    },
];
pub const ONE_PAIR_8_2: [Card; 7] = [
    Card {
        rank: Rank::Rank5,
        suit: Suit::Spades,
    },
    Card {
        rank: Rank::Ace,
        suit: Suit::Hearts,
    },
    Card {
        rank: Rank::Rank8,
        suit: Suit::Spades,
    },
    Card {
        rank: Rank::Rank4,
        suit: Suit::Spades,
    },
    Card {
        rank: Rank::Rank3,
        suit: Suit::Diamonds,
    },
    Card {
        rank: Rank::Rank2,
        suit: Suit::Diamonds,
    },
    Card {
        rank: Rank::Rank8,
        suit: Suit::Clubs,
    },
];
pub const ONE_PAIR_HC8: [Card; 5] = [
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
pub const ONE_PAIR_HCJ: [Card; 5] = [
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
        rank: Rank::Jack,
        suit: Suit::Spades,
    },
];
pub const TWO_PAIR: [Card; 5] = [
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
pub const THREE_OF_A_KIND: [Card; 5] = [
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
pub const STRAIGHT: [Card; 5] = [
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
pub const FLUSH: [Card; 5] = [
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
pub const FULL_HOUSE: [Card; 5] = [
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
pub const FOUR_OF_A_KIND: [Card; 5] = [
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
pub const STRAIGHT_FLUSH: [Card; 5] = [
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
