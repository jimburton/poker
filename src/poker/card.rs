#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Rank {
    Rank2 = 2,
    Rank3 = 3,
    Rank4 = 4,
    Rank5 = 5,
    Rank6 = 6,
    Rank7 = 7,
    Rank8 = 8,
    Rank9 = 9,
    Rank10 = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}
/// Helper methods
impl Rank {
    //  get the numerical value of the rank for continuity checks.
    pub fn value(&self) -> u8 {
        *self as u8
    }

    pub fn values() -> [Rank; 13] {
        [
            Rank::Rank2,
            Rank::Rank3,
            Rank::Rank4,
            Rank::Rank5,
            Rank::Rank6,
            Rank::Rank7,
            Rank::Rank8,
            Rank::Rank9,
            Rank::Rank10,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ]
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
pub enum Suit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

impl Suit {
    pub fn values() -> [Suit; 4] {
        [Suit::Clubs, Suit::Spades, Suit::Diamonds, Suit::Hearts]
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Hand {
    HighCard(Rank),
    OnePair(Rank),
    TwoPair(Rank, Rank),
    ThreeOfAKind(Rank),
    Straight(Rank), // highestrank of the straight
    Flush(Rank, Rank, Rank, Rank, Rank),
    FullHouse(Rank, Rank),
    FourOfAKind(Rank),
    StraightFlush(Rank), // highest rank of the flush
}

pub fn new_deck() -> Vec<Card> {
    Rank::values()
        .iter()
        .flat_map(|i| Suit::values().map(move |j| Card { rank: *i, suit: j }))
        .collect()
}
