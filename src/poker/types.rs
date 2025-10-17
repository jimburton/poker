use rand::{rng, seq::SliceRandom as _};
use std::collections::HashMap;

use num_traits::ToPrimitive;

use crate::poker::hand::best_hand;

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

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub hole: Option<(Card, Card)>,
    pub bet: usize,
    pub bank_roll: usize,
    pub all_in: bool,
    pub folded: bool,
}

impl Player {
    pub fn build(name: &str, bank_roll: usize) -> Self {
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
    pub dealer: Option<String>,
    pub current_player: Option<Player>,
    pub buy_in: usize,
    pub call: usize,
    pub pot: usize,
    pub side_pot: usize,
    pub stage: Stage,
    pub deck: Vec<Card>,
    pub community_cards: Vec<Card>,
    pub num_players: u8,
}

impl Game {
    pub fn build(buy_in: usize, num_players: u8) -> Self {
        let mut game = Game {
            players: HashMap::new(),
            dealer: None,
            current_player: None,
            buy_in,
            call: 0,
            pot: 0,
            side_pot: 0,
            stage: Stage::Blinds,
            deck: Vec::new(),
            community_cards: Vec::new(),
            num_players,
        };
        let mut deck = new_deck();
        let mut rng = rng();
        deck.shuffle(&mut rng);
        game.deck = deck;

        game
    }

    pub fn full(&self) -> bool {
        let num_players = self
            .num_players
            .to_usize()
            .expect("Could not convert num_players to usize");
        self.players.len() == num_players
    }

    pub fn add_player(&mut self, name: &str, bank_roll: usize) -> Result<(), &'static str> {
        if self.full() {
            return Err("Cannot add more players.");
        }
        let p = Player::build(name, bank_roll);
        self.players.insert(name.to_string(), p);
        if self.players.keys().len() == 0 {
            self.dealer = Some(name.to_string());
        }
        Ok(())
    }

    pub fn play(&mut self) {
        self.players_buy_in();
        self.deal_hole_cards();
        self.place_bets();
        self.deal_flop();
        self.place_bets();
        self.deal_turn();
        self.place_bets();
        self.deal_river();
        self.place_bets();
        self.showdown();
    }

    pub fn players_buy_in(&mut self) {
        self.players.iter_mut().for_each(|(_, p)| {
            if p.bank_roll > self.buy_in {
                p.bank_roll -= self.buy_in;
                self.pot += self.buy_in;
            }
        });
    }

    pub fn deal_hole_cards(&mut self) {
        let mut deck = self.deck.clone();
        self.players.iter_mut().for_each(|(_, p)| {
            let c1 = deck.pop().unwrap();
            let c2 = deck.pop().unwrap();
            p.hole = Some((c1, c2));
        });
        self.deck = deck;
    }

    pub fn deal_flop(&mut self) {
        let mut deck = self.deck.clone();
        let _burn = deck.pop().unwrap();
        let c1 = deck.pop().unwrap();
        let c2 = deck.pop().unwrap();
        let c3 = deck.pop().unwrap();
        self.community_cards.push(c1);
        self.community_cards.push(c2);
        self.community_cards.push(c3);
        self.deck = deck;
    }

    pub fn deal_turn(&mut self) {
        let mut deck = self.deck.clone();
        let _burn = deck.pop().unwrap();
        let c1 = deck.pop().unwrap();
        self.community_cards.push(c1);
        self.deck = deck;
    }

    pub fn deal_river(&mut self) {
        let mut deck = self.deck.clone();
        let _burn = deck.pop().unwrap();
        let c1 = deck.pop().unwrap();
        self.community_cards.push(c1);
        self.deck = deck;
    }

    pub fn place_bets(&mut self) {
        self.players.iter_mut().for_each(|(_, p)| {
            if self.call == 0 {
                self.call = 10
            }
            if p.bank_roll > self.call {
                p.bank_roll -= self.call;
                self.pot += self.call;
            }
        });
    }

    pub fn showdown(&mut self) {
        let mut hands: Vec<(String, Hand)> = Vec::new();
        self.players.iter_mut().for_each(|(_, p)| {
            let (c1, c2) = p.hole.unwrap();
            let mut ccards = self.community_cards.clone();
            ccards.push(c1);
            ccards.push(c2);
            let best_hand = best_hand(&ccards);
            let name = p.name.clone();
            hands.push((name, best_hand));
        });
        hands.sort();
        println!("Hands: {:?}", hands);
    }
}

fn new_deck() -> Vec<Card> {
    Rank::values()
        .iter()
        .flat_map(|i| Suit::values().map(move |j| Card { rank: *i, suit: j }))
        .collect()
}
