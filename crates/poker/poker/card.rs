/// Types and functions relating to cards.
use std::fmt::{self, Display};

/// The rank of a card.
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
/// Implementation of Display trait for Rank.
impl Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = match self.value() {
            2..=10 => format!("{}", self.value()),
            11 => "Jack".to_string(),
            12 => "Queen".to_string(),
            13 => "King".to_string(),
            14 => "Ace".to_string(),
            _ => "Unknown".to_string(),
        };
        write!(f, "{}", val)
    }
}
/// Rank helper methods
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

/// The suit of a card.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy, Hash)]
pub enum Suit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}
/// Helper method for Suit.
impl Suit {
    pub fn values() -> [Suit; 4] {
        [Suit::Clubs, Suit::Spades, Suit::Diamonds, Suit::Hearts]
    }
}
/// Implementation of Display trait for Suit.
impl Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suit::Clubs => write!(f, "Clubs"),
            Suit::Spades => write!(f, "Spades"),
            Suit::Diamonds => write!(f, "Diamonds"),
            Suit::Hearts => write!(f, "Hearts"),
        }
    }
}

/// A card has a rank and a suit.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}
/// Implementation of Display trait for Card.
impl Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}

/// A poker hand, ranked from lowest to highest. Assuming there are no wild cards allowed,
/// and so no five of a kind.
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
/// Implementation of Display trait for Hand.
impl Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Hand::HighCard(r) => write!(f, "High Card ({})", r),
            Hand::OnePair(r) => write!(f, "One Pair ({})", r),
            Hand::TwoPair(r1, r2) => write!(f, "Two Pair ({} and {})", r1, r2),
            Hand::ThreeOfAKind(r) => write!(f, "Three of a Kind ({})", r),
            Hand::Straight(r) => write!(f, "Straight (ending {})", r),
            Hand::Flush(r1, _r2, _r3, _r4, r5) => write!(f, "Flush ({} to {})", r1, r5),
            Hand::FullHouse(r1, r2) => write!(f, "Full House ({} {})", r1, r2),
            Hand::FourOfAKind(r) => write!(f, "Four of a Kind ({})", r),
            Hand::StraightFlush(r) => write!(f, "Straight Flush (ending {})", r),
        }
    }
}

/// Get a new unshuffled deck of 52 cards.
pub fn new_deck() -> Vec<Card> {
    Rank::values()
        .iter()
        .flat_map(|i| Suit::values().map(move |j| Card { rank: *i, suit: j }))
        .collect()
}
