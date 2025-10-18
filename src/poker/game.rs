use crate::poker::card::{new_deck, Card, Hand};
use crate::poker::compare::{best_hand, compare_hands};
use num_traits::ToPrimitive;
use rand::rng;
use rand::seq::SliceRandom;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Winner {
    Winner {
        name: String,
        hand: Hand,
        cards: Vec<Card>,
    },
    Draw(Vec<(String, Hand, Vec<Card>)>),
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

pub fn new_game(buy_in: usize, num_players: u8) -> Game {
    Game::build(buy_in, num_players)
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

    fn players_buy_in(&mut self) {
        self.players.iter_mut().for_each(|(_, p)| {
            if p.bank_roll > self.buy_in {
                p.bank_roll -= self.buy_in;
                self.pot += self.buy_in;
            }
        });
    }

    fn deal_hole_cards(&mut self) {
        let mut deck = self.deck.clone();
        self.players.iter_mut().for_each(|(_, p)| {
            let c1 = deck.pop().unwrap();
            let c2 = deck.pop().unwrap();
            p.hole = Some((c1, c2));
        });
        self.deck = deck;
    }

    fn deal_flop(&mut self) {
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

    fn deal_turn(&mut self) {
        let mut deck = self.deck.clone();
        let _burn = deck.pop().unwrap();
        let c1 = deck.pop().unwrap();
        self.community_cards.push(c1);
        self.deck = deck;
    }

    fn deal_river(&mut self) {
        let mut deck = self.deck.clone();
        let _burn = deck.pop().unwrap();
        let c1 = deck.pop().unwrap();
        self.community_cards.push(c1);
        self.deck = deck;
    }

    fn place_bets(&mut self) {
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

    /// Determines the winner(s) of the hand.
    fn showdown(&mut self) {
        // 1. Calculate the best hand for each non-folded player.
        let mut hands: Vec<(String, Hand, Vec<Card>)> = self
            .players
            .iter() // Use iter() since we don't need to mutate Player state here
            .filter_map(|(_, p)| {
                // Only consider players who haven't folded
                if p.folded {
                    return None;
                }

                let (c1, c2) = p.hole.expect("Hole cards should be dealt before showdown");

                // Collect all 7 cards (2 hole + 5 community)
                let mut all_cards = self.community_cards.clone();
                all_cards.push(c1);
                all_cards.push(c2);

                let best_hand = best_hand(&all_cards);

                // Returns an owned tuple for comparison
                Some((p.name.clone(), best_hand, all_cards))
            })
            .collect();
        dbg!("All hands: {:?}", &hands);

        // Handle cases where 0 or 1 players remain (the last player standing wins)
        if hands.len() < 2 {
            if let Some((name, hand, cards)) = hands.pop() {
                let winner = Winner::Winner { name, hand, cards };
                println!("Winner (last player standing): {:?}", winner);
            } else {
                println!("No players remaining to determine winner.");
            }
            return;
        }

        // 2. Initialize the winner with the first player's hand, consuming it from the vector.
        let mut winner: Winner = {
            let (name, hand, cards) = hands.remove(0);
            Winner::Winner { name, hand, cards }
        };

        // 3. Compare the current winner against all remaining hands.
        for (challenger_name, challenger_hand, challenger_cards) in hands {
            // Match is used to consume the existing winner state and re-assign
            // the result back to 'winner' for the next iteration.
            winner = match winner {
                Winner::Winner {
                    name: w_name,
                    hand: w_hand,
                    cards: w_cards,
                } => {
                    // Compare the current winner (w_...) against the challenger (challenger_...)
                    compare_hands(
                        (challenger_name, challenger_hand, challenger_cards),
                        (w_name, w_hand, w_cards),
                    )
                }
                Winner::Draw(mut draw_winners) => {
                    // If it's a draw, compare the challenger against the best hand in the draw group.
                    // We can safely assume the first element of a Draw is the benchmark.
                    let (w_name_benchmark, w_hand_benchmark, w_cards_benchmark) =
                        draw_winners.pop().unwrap();

                    let comparison_result = compare_hands(
                        (
                            challenger_name.clone(),
                            challenger_hand,
                            challenger_cards.clone(),
                        ),
                        (
                            w_name_benchmark.clone(),
                            w_hand_benchmark,
                            w_cards_benchmark.clone(),
                        ),
                    );

                    // Put the benchmark hand back for future comparisons or draw outcome
                    draw_winners.push((w_name_benchmark, w_hand_benchmark, w_cards_benchmark));

                    match comparison_result {
                        Winner::Winner {
                            name: n,
                            hand: h,
                            cards: c,
                        } => {
                            if n == challenger_name {
                                // Challenger is better than the benchmark (and thus all previous winners)
                                Winner::Winner {
                                    name: n,
                                    hand: h,
                                    cards: c,
                                }
                            } else {
                                // Challenger is worse than the benchmark, keep the existing draw group
                                Winner::Draw(draw_winners)
                            }
                        }
                        Winner::Draw(_) => {
                            // Challenger ties with the benchmark, add challenger to the draw group
                            draw_winners.push((challenger_name, challenger_hand, challenger_cards));
                            Winner::Draw(draw_winners)
                        }
                    }
                }
            };
        }

        dbg!("Final Showdown Result: {:?}", winner);
        // TODO distribute the pot based on the 'winner' state.
    }
}
