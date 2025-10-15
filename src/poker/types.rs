use std::collections::HashMap;

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
/// Helper method to get the numerical value of the rank for continuity checks.
impl Rank {
    pub fn value(&self) -> u8 {
        *self as u8
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
pub enum Suit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
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

#[derive(Debug)]
pub enum Winner {
    Winner { name: String, hand: Hand },
    Draw,
}

#[derive(Debug)]
pub enum Stage {
    Blinds,
    Hole,
    PreFlop,
    Turn,
    River,
    ShowDown,
}

#[derive(Debug)]
pub enum Bet {
    Check,
    Hold(u64),
    Raise(u64),
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub hole: Option<(Card, Card)>,
    pub bet: u64,
    pub bank_roll: u64,
    pub all_in: bool,
    pub folded: bool,
}

impl Player {
    pub fn new(name: &str, bank_roll: u64) -> Self {
        Player {
            name: name.to_string(),
            bank_roll,
            hole: None,
            bet: 0,
            all_in: false,
            folded: false,
        }
    }
}

#[derive(Debug)]
pub struct Game {
    pub players: HashMap<String, Player>,
    pub dealer: Option<Player>,
    pub current_player: Option<Player>,
    pub buy_in: u64,
    pub call: u64,
    pub pot: u64,
    pub side_pot: u64,
    pub stage: Stage,
    pub deck: Vec<Card>,
}

impl Game {
    pub fn new(buy_in: u64) -> Self {
        Game {
            players: HashMap::new(),
            dealer: None,
            current_player: None,
            buy_in,
            call: 0,
            pot: 0,
            side_pot: 0,
            stage: Stage::Blinds,
            deck: Vec::new(),
        }
    }
}
