use crate::poker::betting_strategy::BettingStrategy;
use crate::poker::card::{new_deck, Card, Hand};
use crate::poker::compare::{best_hand, compare_hands};
use crate::poker::player::{
    AutoPlayer, Msg, Player, PlayerHand, RoundWinnerInfo, Winner, WinnerInfo,
};
use crate::poker::rotate_vector;
use num_traits::ToPrimitive;
use rand::rng;
use rand::seq::SliceRandom;
use std::collections::HashMap;

use super::player::Update;

/// Datatypes and functions for the game and individual rounds.

/// Enum for representing the stage of a round.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Stage {
    Blinds,
    Hole,
    PreFlop,
    Flop,
    Turn,
    River,
    ShowDown,
}

/// Enum fro representing a bet.
#[derive(Debug, Clone)]
pub enum Bet {
    Fold,
    Check,
    Call,
    Raise(usize),
    AllIn(usize),
}

/// Struct for a side pot.
#[derive(Debug, Clone)]
struct SidePot {
    players: Vec<String>,
    pot: usize,
}

/// Struct for the game.
#[derive(Debug)]
pub struct Game {
    players: HashMap<String, Box<dyn Player>>,
    players_order: Vec<String>,
    dealer: Option<String>,
    buy_in: usize,
    small_blind: usize,
    big_blind: usize,
    pot: usize,
    side_pots: Vec<SidePot>,
    deck: Vec<Card>,
    community_cards: Vec<Card>,
    max_players: u8,
    winner: Option<Winner>,
    stage: Stage,
    num_rounds: usize,
}

/// Implementation for the Game struct.
impl Game {
    pub fn build(big_blind: usize, max_players: u8) -> Self {
        let mut game = Game {
            players: HashMap::new(),
            players_order: Vec::new(),
            dealer: None,
            buy_in: 100 * big_blind,
            small_blind: big_blind / 2,
            big_blind,
            pot: 0,
            side_pots: Vec::new(),
            deck: Vec::new(),
            community_cards: Vec::new(),
            max_players,
            winner: None,
            stage: Stage::Blinds,
            num_rounds: 0,
        };
        let mut deck = new_deck();
        let mut rng = rng();
        deck.shuffle(&mut rng);
        game.deck = deck;

        game
    }

    /// Predicate function for the game having the full amount of players.
    fn full(&self) -> bool {
        let num_players = self
            .max_players
            .to_usize()
            .expect("Could not convert num_players to usize");
        self.players.len() == num_players
    }

    /// Allows a player to join the game. The player's bank roll is set to the buy in amount.
    pub fn join(&mut self, mut player: impl Player + 'static) -> Result<(), &'static str> {
        if self.full() {
            return Err("Cannot add more players.");
        }
        let name = player.get_name();
        println!("Player joining: {}", name.clone());
        player.set_bank_roll(self.buy_in);
        self.players.insert(name.clone(), Box::new(player));
        self.players_order.push(name);
        Ok(())
    }

    /// Play a game.
    pub fn play(&mut self) -> WinnerInfo {
        println!("Playing game with: {:?}", self.players);
        while self.players.len() > 1 {
            self.play_round();
            self.reset_after_round();
            self.num_rounds += 1;
        }
        let w = self.get_winner();
        self.announce_winner_game(&w);
        w
    }

    /// Determine the winner at the end of the game.
    fn get_winner(&self) -> WinnerInfo {
        let winner_opt = self.players_order.first();
        if let Some(name) = winner_opt {
            let winner = self.players.get(name).unwrap();
            let name = winner.get_name().clone();
            let winnings = winner.get_bank_roll();
            let num_rounds = self.num_rounds;
            WinnerInfo {
                name,
                num_rounds,
                winnings,
            }
        } else {
            panic!("Announcing winner but they have been removed...")
        }
    }

    /// Announce the winner at the end of the game.
    fn announce_winner_game(&self, winner: &WinnerInfo) {
        println!(
            "Congratulations {}, you played {} rounds and won with a bank roll of {}.",
            winner.name, winner.num_rounds, winner.winnings
        );
    }

    /// Announce the winner at the end of the game.
    fn announce_winner_round(&self) {
        let w = self.winner.as_ref().unwrap();
        let msg = Msg::MsgWinner(w.clone());
        for player in self.players.values() {
            player.update(&msg);
        }
    }

    /// Set the name of the dealer and reorder the players_order list
    /// so that the player to the left of the dealer is at the front.
    fn order_players(&mut self) {
        if self.stage == Stage::Blinds {
            let players_order: Vec<String> = self.players_order.clone();
            self.dealer = Some(players_order.first().unwrap().clone());
        }
        let dealer = self.dealer.as_ref().clone();
        if self.players.contains_key(dealer.unwrap()) {
            let pos = self
                .players_order
                .iter()
                .position(|n| n == dealer.unwrap())
                .unwrap();
            let current_i = pos + 1 % self.players_order.len();
            self.players_order = rotate_vector(&self.players_order, current_i);
        } else {
            self.dealer = Some(self.players_order.first().unwrap().clone());
        }
    }

    /// Play a round.
    fn play_round(&mut self) {
        self.order_players();
        self.ante_up();
        self.stage = Stage::Hole;
        self.deal_hole_cards();
        self.stage = Stage::PreFlop;
        self.place_bets();
        self.stage = Stage::Flop;
        self.deal_flop();
        self.place_bets();
        self.stage = Stage::Turn;
        self.deal_turn();
        self.place_bets();
        self.stage = Stage::River;
        self.deal_river();
        self.place_bets();
        self.stage = Stage::ShowDown;
        self.showdown();
        self.distribute_pots();
        self.announce_winner_round();
    }

    /// Each player pays the small or big blind at the beginning of each round,
    /// or chooses not to if they don't want to take part in this round.
    /// The player to the left of the dealer pays the small blind, everyone else
    /// pays the big blind.
    fn ante_up(&mut self) {
        if self.players_order.is_empty() {
            return;
        }

        let players_order = self.players_order.clone();

        // Handle the first player (Small Blind) in a restricted scope.
        // This allows the mutable reference 'first_p' to drop immediately.
        {
            let left_of_dealer: &String = players_order.first().unwrap();

            // Mutably borrow the player and execute the action
            if let Some(first_p) = self.players.get_mut(left_of_dealer) {
                if let Ok(blind) = first_p.ante_up(self.small_blind) {
                    self.pot += blind;
                }
                // NB: player marks themself as folded if they responded negatively
                // or as all in if their bank roll was less than the blind.
            }
        } // <- Mutable borrow of self.players drops here.

        // Handle the remaining players.
        // We can now mutably borrow players inside the loop without conflict.
        players_order[1..].iter().for_each(|name| {
            // Mutably borrow the current player 'p'
            if let Some(p) = self.players.get_mut(name) {
                // The players map is only borrowed for the duration of this 'if let' block.
                if let Ok(blind) = p.ante_up(self.big_blind) {
                    self.pot += blind
                }
            }
        });
    }

    /// Each player receives two hole cards after ante up.
    fn deal_hole_cards(&mut self) {
        let mut deck = self.deck.clone();
        self.players.iter_mut().for_each(|(_, p)| {
            let c1 = deck.pop().unwrap();
            let c2 = deck.pop().unwrap();
            p.accept_hole_cards(Some((c1, c2)));
        });
        self.deck = deck;
    }

    /// Burn one card and deal the first three three community cards.
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

    /// Burn one card and deal the fourth community card.
    fn deal_turn(&mut self) {
        let mut deck = self.deck.clone();
        let _burn = deck.pop().unwrap();
        let c1 = deck.pop().unwrap();
        self.community_cards.push(c1);
        self.deck = deck;
    }

    /// Burn one card and deal the fifth and final community card.
    fn deal_river(&mut self) {
        let mut deck = self.deck.clone();
        let _burn = deck.pop().unwrap();
        let c1 = deck.pop().unwrap();
        self.community_cards.push(c1);
        self.deck = deck;
    }

    /// Players are given the opportunity to bet. If a player raises the bet, every
    /// other player must call or raise again.
    fn place_bets(&mut self) {
        println!("Placing bets: {:?}", self.players);
        // names of players who have not folded
        let not_folded: Vec<(String, bool)> = self
            .players
            .values()
            .filter(|p| !p.get_folded())
            .map(|p| (p.get_name().clone(), p.get_all_in()))
            .collect();
        // names of players who have not folded and are not all in
        let mut not_all_in: Vec<String> = not_folded
            .iter()
            .filter(|(_name, all_in)| !all_in)
            .map(|(name, _all_in)| name.clone())
            .collect();
        // The players who will be betting, in the right order
        let mut players: Vec<String> = Vec::new();
        for name in self.players_order.clone() {
            if not_folded.iter().any(|(n, _b)| n == &name) {
                players.push(name);
            }
        }
        if players.is_empty() {
            return;
        }

        let mut current: usize = 0;
        let mut target: String = players.first().unwrap().clone();

        let mut done: bool = false;
        let mut target_placed_bet: bool = false;
        let mut call: usize = 0;
        let min = self.big_blind;
        let mut cycle: u8 = 0; // the number of times players have been given a chance to bet in this round

        while !done {
            // Use the cloned players_order for iteration indexing
            let current_name = &players[current % players.len()];

            // Mutable borrow of self.players is short-lived within this loop iteration.
            let p = self.players.get_mut(current_name).unwrap();

            // Compare the player's name with the owned target String.
            if p.get_name() == target && target_placed_bet {
                done = true;
            } else {
                if p.get_name() == target {
                    target_placed_bet = true;
                    cycle += 1;
                }
                let ccards = self.community_cards.clone();
                let mut bet_opt: Option<Bet> = None;
                bet_opt = p.place_bet(call, min, ccards, self.stage, cycle);

                let bet = bet_opt.unwrap();

                match bet {
                    Bet::Fold => {
                        p.set_folded(true);
                        players.remove(current);
                        continue; // continue without incrementing current
                    }
                    Bet::Check => {
                        if call > 0 {
                            panic!("Misbehaving client checked when there was an outstanding bet.");
                        }
                    }
                    Bet::Call => {
                        self.pot += call;
                    }
                    Bet::Raise(raise) => {
                        if raise > call {
                            if !self.side_pots.is_empty() {
                                // Must get a mutable reference to side_pots here
                                let side_pot = self.side_pots.get_mut(0).unwrap();
                                side_pot.pot += raise;
                            } else {
                                self.pot += raise;
                            }
                            // raise is the new total amount to match/beat
                            call = raise;
                            // 3. FIX: Clone the player's name into the owned 'target' String.
                            target = p.get_name().clone();
                        } else {
                            dbg!("Tried to raise less than call.");
                            p.set_folded(true);
                        }
                    }
                    Bet::AllIn(bet) => {
                        self.pot += bet;

                        // not_all_in now contains owned Strings, so we can search by value.
                        if let Some(index) =
                            not_all_in.iter().position(|value| value == &p.get_name())
                        {
                            not_all_in.swap_remove(index);
                        }

                        // Clone the list for the new side pot's players
                        let new_side_pot = SidePot {
                            players: not_all_in.clone(),
                            pot: 0,
                        };
                        self.side_pots.push(new_side_pot);
                    }
                }
                let update = Msg::MsgBet(Update {
                    player: p.get_name().clone(),
                    bet,
                });
                self.players.iter().for_each(|(_name, p)| {
                    p.update(&update);
                });

                // ensure 'current' wraps correctly.
                current = (current + 1) % players.len();
            }
        }
    }

    fn update_interactive_players(&self, update: &Msg) {
        self.players.iter().for_each(|(_name, p)| {
            p.update(update);
        });
    }

    /// Determines the winner(s) of the round.
    fn showdown(&mut self) {
        // Calculate the best hand for each non-folded player.
        let mut hands: Vec<PlayerHand> =
            self.names_to_hands(self.players.keys().cloned().collect());
        // Handle cases where 0 or 1 players remain (the last player standing wins)
        if hands.len() < 2 {
            if let Some(PlayerHand {
                name,
                best_hand,
                cards,
            }) = hands.pop()
            {
                let winner = Winner::Winner {
                    name,
                    hand: best_hand,
                    cards,
                };
                self.winner = Some(winner);
            } else {
                dbg!("No players remaining to determine winner.");
            }
            return;
        }

        let winner = Game::determine_winner(hands);
        self.winner = Some(winner);
    }

    /// Take a vector of player names and return a vector of (name, hand, cards),
    /// where hand is their best hand and cards is their hole cards plus community cards.
    /// Result contains only non-folded players.
    fn names_to_hands(&self, names: Vec<String>) -> Vec<PlayerHand> {
        // Calculate the best hand for each non-folded player.
        let hands: Vec<PlayerHand> = self
            .players
            .iter() // Use iter() since we don't need to mutate Player state here
            .filter_map(|(_, p)| {
                // Only consider players who haven't folded
                if p.get_folded() || !names.contains(&p.get_name()) {
                    return None;
                }

                let (c1, c2) = p
                    .get_hole()
                    .expect("Hole cards should be dealt before calling names_to_hands");

                // Collect all 7 cards (2 hole + 5 community)
                let mut all_cards = self.community_cards.clone();
                all_cards.push(c1);
                all_cards.push(c2);

                let best_hand = best_hand(&all_cards);
                Some(PlayerHand {
                    name: p.get_name().clone(),
                    best_hand,
                    cards: all_cards,
                })
            })
            .collect();
        hands
    }

    /// Determine winner(s) from vector of (name, best_hand, cards) tuples.
    fn determine_winner(mut hands: Vec<PlayerHand>) -> Winner {
        // Handle cases where 0 or 1 players remain (the last player standing wins)
        if hands.len() < 2 {
            if let Some(PlayerHand {
                name,
                best_hand,
                cards,
            }) = hands.pop()
            {
                return Winner::Winner {
                    name,
                    hand: best_hand,
                    cards,
                };
            } else {
                dbg!("No players remaining to determine winner.");
            }
        }

        // Compare the hands.

        // Initialize winner with the first player's hand, consuming it from the vector.
        let mut winner: Winner = {
            let PlayerHand {
                name,
                best_hand,
                cards,
            } = hands.remove(0);
            Winner::Winner {
                name,
                hand: best_hand,
                cards,
            }
        };

        // Compare current winner against all remaining hands.
        for PlayerHand {
            name: challenger_name,
            best_hand: challenger_hand,
            cards: challenger_cards,
        } in hands
        {
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
                        // Challenger data is moved here
                        PlayerHand {
                            name: challenger_name,
                            best_hand: challenger_hand,
                            cards: challenger_cards,
                        },
                        PlayerHand {
                            name: w_name,
                            best_hand: w_hand,
                            cards: w_cards,
                        },
                    )
                }
                Winner::Draw(mut draw_winners) => {
                    // It's a draw, compare the challenger against the best hand in the draw group.
                    // Assume the first element of a Draw is the benchmark.
                    let PlayerHand {
                        name: w_name_benchmark,
                        best_hand: w_hand_benchmark,
                        cards: w_cards_benchmark,
                    } = draw_winners.pop().unwrap();

                    let comparison_result = compare_hands(
                        PlayerHand {
                            name: challenger_name.clone(),
                            best_hand: challenger_hand,
                            cards: challenger_cards.clone(),
                        },
                        PlayerHand {
                            name: w_name_benchmark.clone(),
                            best_hand: w_hand_benchmark,
                            cards: w_cards_benchmark.clone(),
                        },
                    );

                    // Put the benchmark hand back for future comparisons or draw outcome
                    draw_winners.push(PlayerHand {
                        name: w_name_benchmark,
                        best_hand: w_hand_benchmark,
                        cards: w_cards_benchmark,
                    });

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
                            // Challenger ties with the benchmark, add challenger to the draw group.
                            // The original (un-cloned) challenger values are now moved here.
                            draw_winners.push(PlayerHand {
                                name: challenger_name,
                                best_hand: challenger_hand,
                                cards: challenger_cards,
                            });
                            Winner::Draw(draw_winners)
                        }
                    }
                }
            };
        }
        winner
    }

    /// Distributes the pot and side pot to the winner(s).
    ///
    /// TODO keep track of chips being lost due to truncating division.
    fn distribute_pots(&mut self) {
        let winner = self.winner.clone();
        let main_pot = self.pot;
        let side_pots = self.side_pots.clone();
        let ccards = self.community_cards.clone();
        // details of not folded players: names, bests hands, full sets of cards and whether they are all in
        let not_folded: Vec<(PlayerHand, bool)> = self
            .players
            .values()
            .filter(|p| !p.get_folded())
            .map(|p| {
                let (c1, c2) = p.get_hole().unwrap();
                let mut cards = ccards.clone();
                cards.extend(vec![c1, c2]);
                (
                    PlayerHand {
                        name: p.get_name().clone(),
                        best_hand: best_hand(&cards),
                        cards,
                    },
                    p.get_all_in(),
                )
            })
            .collect();
        // not folded and not all in
        let not_all_in: Vec<PlayerHand> = not_folded
            .iter()
            .filter(|(_ph, all_in)| !all_in)
            .map(
                |(
                    PlayerHand {
                        name,
                        best_hand,
                        cards,
                    },
                    _all_in,
                )| PlayerHand {
                    name: name.clone(),
                    best_hand: best_hand.clone(),
                    cards: cards.clone(),
                },
            )
            .collect();
        let not_folded_clone = not_folded.clone();
        // store winnings during distribution algorithm, allocate at end
        let mut winnings: HashMap<String, usize> = HashMap::new();
        for (ph, _b) in not_folded {
            winnings.insert(ph.name, 0);
        }
        if let Some(w) = winner {
            match w {
                Winner::Winner { name, .. } => {
                    let winner_name = name.clone();
                    if not_folded_clone.iter().any(|(ph, _all_in)| ph.name == name) {
                        // winner is not folded
                        // distribute the main pot
                        match winnings.get_mut(&winner_name) {
                            Some(value) => *value += main_pot, // If the key exists, update the value
                            None => {
                                panic!("Key '{}' does not exist in the HashMap.", winner_name)
                            }
                        }
                        if not_all_in.iter().any(|ph| ph.name == winner_name) {
                            // winner is not all in, they win the side pots too
                            let side_pots: usize = self.side_pots.iter().map(|sp| sp.pot).sum();
                            match winnings.get_mut(&winner_name) {
                                Some(value) => *value += side_pots, // If the key exists, update the value
                                None => {
                                    panic!("Key '{}' does not exist in the HashMap.", winner_name)
                                }
                            }
                        } else {
                            // winner is all in, they only win side pots they contributed to
                            // distribute side pots
                            for sp in side_pots {
                                // possible winners
                                let candidates: Vec<PlayerHand> = not_folded_clone
                                    .iter()
                                    .filter(|(ph, _all_in)| sp.players.contains(&ph.name))
                                    .map(|(ph, _all_in)| PlayerHand {
                                        name: ph.name.clone(),
                                        best_hand: ph.best_hand,
                                        cards: ph.cards.clone(),
                                    })
                                    .collect();
                                if candidates.is_empty() {
                                    // everyone in this side pot has folded so the winnings go to the winner of the main pot
                                    match winnings.get_mut(&winner_name) {
                                        Some(value) => *value += sp.pot, // If the key exists, update the value
                                        None => {
                                            panic!("Key '{}' does not exist in the HashMap.", name)
                                        }
                                    }
                                } else {
                                    // players who participated in this side pot are still in the round
                                    let w = Game::determine_winner(candidates);
                                    match w {
                                        // single winner for this side pot
                                        Winner::Winner { name, .. } => {
                                            match winnings.get_mut(&name) {
                                                Some(value) => *value += sp.pot, // If the key exists, update the value
                                                None => panic!(
                                                    "Key '{}' does not exist in the HashMap.",
                                                    name
                                                ),
                                            }
                                        }
                                        // multiple winners for this side pot
                                        Winner::Draw(winners) => {
                                            let pot_share = sp.pot / winners.len();
                                            for PlayerHand {
                                                name,
                                                best_hand: _,
                                                cards: _,
                                            } in winners
                                            {
                                                match winnings.get_mut(&name) {
                                                    Some(value) => *value += pot_share, // If the key exists, update the value
                                                    None => panic!(
                                                        "Key '{}' does not exist in the HashMap.",
                                                        name
                                                    ),
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        dbg!("Winner not in not_folded.");
                    }
                }
                Winner::Draw(winners) => {
                    // distribute main pot
                    let main_pot_share = main_pot / winners.len();
                    for PlayerHand {
                        name,
                        best_hand: _,
                        cards: _,
                    } in winners.clone()
                    {
                        match winnings.get_mut(&name) {
                            Some(value) => *value += main_pot_share,
                            None => panic!("Key '{}' does not exist in the HashMap.", name),
                        }
                    }
                    //distribute side pots
                    for sp in side_pots {
                        // possible winners
                        let candidates: Vec<PlayerHand> = not_folded_clone
                            .iter()
                            .filter(|(ph, _all_in)| sp.players.contains(&ph.name))
                            .map(|(ph, _all_in)| PlayerHand {
                                name: ph.name.clone(),
                                best_hand: ph.best_hand,
                                cards: ph.cards.clone(),
                            })
                            .collect();
                        if candidates.is_empty() {
                            // everyone who contributed to this side pot has folded, the winners share the pot
                            for PlayerHand {
                                name,
                                best_hand: _,
                                cards: _,
                            } in winners.clone()
                            {
                                match winnings.get_mut(&name) {
                                    Some(value) => *value += sp.pot,
                                    None => {
                                        panic!("Key '{}' does not exist in the HashMap.", name)
                                    }
                                }
                            }
                        } else {
                            // there are unfolded players who contributed to this side pot
                            let w = Game::determine_winner(candidates);
                            match w {
                                // single winner for this side pot
                                Winner::Winner { name, .. } => match winnings.get_mut(&name) {
                                    Some(value) => *value += sp.pot,
                                    None => {
                                        panic!("Key '{}' does not exist in the HashMap.", name)
                                    }
                                },
                                // multiple winners for this side pot
                                Winner::Draw(winners) => {
                                    let pot_share = sp.pot / winners.len();
                                    for PlayerHand {
                                        name,
                                        best_hand: _,
                                        cards: _,
                                    } in winners
                                    {
                                        match winnings.get_mut(&name) {
                                            Some(value) => *value += pot_share,
                                            None => panic!(
                                                "Key '{}' does not exist in the HashMap.",
                                                name
                                            ),
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // distribute winnings
            for (name, pot_share) in winnings.clone() {
                if pot_share > 0 {
                    match self.players.get_mut(&name) {
                        Some(value) => value.set_bank_roll(value.get_bank_roll() + pot_share),
                        None => panic!("Key '{}' does not exist in the HashMap.", name),
                    }
                }
            }
            self.pot = 0;
            self.side_pots = Vec::new();
        } else {
            dbg!("Distribute pots called with no winner set.");
        }
    }

    fn reset_after_round(&mut self) {
        self.pot = 0;
        self.side_pots = Vec::new();
        self.community_cards = Vec::new();
        self.deck = new_deck();
        let dealer_name_ref: Option<&String> = self.dealer.as_ref();

        let mut removed_names: Vec<String> = Vec::new();

        // Loop through the players resetting all_in and folded, and collecting
        // the list of ones that need to be removed for lack of money.
        for name in self.players_order.iter() {
            let p = self
                .players
                .get_mut(name)
                .expect("Player in order list not found in map.");
            p.set_all_in(false);
            p.set_folded(false);
            p.accept_hole_cards(None);
            let is_dealer = dealer_name_ref.unwrap() == name;
            // if player doesn't have enough chips to continue, mark for removal
            if (is_dealer && p.get_bank_roll() < self.small_blind)
                || (p.get_bank_roll() < self.big_blind)
            {
                removed_names.push(name.clone());
            }
        }

        // remove player names from self.players_order and Player structs from self.player
        for name in removed_names.iter() {
            self.players.remove(name.as_str());
            if let Some(index) = self.players_order.iter().position(|value| value == name) {
                self.players_order.swap_remove(index);
            }
            self.players.remove(name);
        }
    }
}

/// Create a new game.
pub fn new_game(buy_in: usize, num_players: u8) -> Game {
    Game::build(buy_in, num_players)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::poker::card::{Card, Hand, Rank, Suit};

    #[test]
    fn test_build() {
        let game = Game::build(10, 5);
        assert!(
            game.deck.len() == 52,
            "Expected game.deck to have 52 cards, was {}",
            game.deck.len()
        );
    }

    #[test]
    fn test_add_too_many_players() {
        let mut game = Game::build(10, 2);
        let _ = game.join(AutoPlayer::build("player1"));
        let _ = game.join(AutoPlayer::build("player2"));
        if let Err(e) = game.join(AutoPlayer::build("player3")) {
            assert!(
                e == "Cannot add more players.",
                "Expected 'Cannot add more players.', was {}",
                e
            );
        } else {
            panic!("Expected add_player to raise an error.");
        }
    }

    #[test]
    fn test_players_receive_bank_roll() {
        let mut game = Game::build(20, 2);
        let _ = game.join(AutoPlayer::build("player1"));
        let _ = game.join(AutoPlayer::build("player2"));
        // each player should receive 100 x big blind
        game.players.values().for_each(|p| {
            assert!(
                p.get_bank_roll() == 100 * 20,
                "Expected new player to receive {}, was {}",
                100 * 20,
                p.get_bank_roll()
            )
        });
    }

    #[test]
    fn test_deal_hole_cards() {
        let mut game = Game::build(20, 2);
        let _ = game.join(AutoPlayer::build("player1"));
        let _ = game.join(AutoPlayer::build("player2"));
        game.deal_hole_cards();
        assert!(
            game.deck.len() == 48,
            "Expected game.deck.len() to be 48, was {}",
            game.deck.len()
        );
        game.players.iter().for_each(|(name, p)| {
            if let Some((c1, c2)) = p.hole {
                assert!(
                    !game.deck.contains(&c1),
                    "Expected {:?} not to be in game.deck but it was.",
                    c1
                );
                assert!(
                    !game.deck.contains(&c2),
                    "Expected {:?} not to be in game.deck but it was.",
                    c2
                );
            } else {
                panic!("Expected hole cards for player {}", name);
            }
        });
    }

    #[test]
    fn test_place_bets_default_strategy() {
        let mut game = Game::build(20, 2);
        let _ = game.join(AutoPlayer::build("player1"));
        let _ = game.join(AutoPlayer::build("player2"));
        game.order_players();
        game.deal_hole_cards();

        // Test with players that don't place bets.
        // Both players will use default strategy and check.
        game.place_bets();
        assert!(game.pot == 0, "Expected game.pot to be 0, was {}", game.pot);
        game.players.iter().for_each(|(_name, p)| {
            assert!(
                p.get_bank_roll() == 2000,
                "Expected p.bank_roll to be 2000, was {}.",
                p.get_bank_roll()
            );
        });

        // Test with a player who bets once.
        let p2 = game.players.get_mut("player2").unwrap();
        // make a strategy that will place a bet if the call is zero
        fn strategy(
            call: usize,
            min: usize,
            bank_roll: usize,
            _cards: Vec<Card>,
            _stage: Stage,
            _cycle: u8,
        ) -> Bet {
            if bank_roll == 0 {
                Bet::Fold
            } else if bank_roll <= call {
                Bet::AllIn(bank_roll)
            } else if call == 0 {
                Bet::Raise(min)
            } else {
                Bet::Call
            }
        }
        p2.set_betting_strategy(strategy);
        game.place_bets();
        assert!(
            game.pot == 40,
            "Expected game.pot to be 40, was {}",
            game.pot
        );
        game.players.iter().for_each(|(_name, p)| {
            assert!(
                p.get_bank_roll() == 1980,
                "Expected p.bank_roll to be 1980, was {}.",
                p.get_bank_roll()
            );
        });
    }

    // A betting strategy that will place a bet if the call is zero
    fn test_strategy(
        call: usize,
        min: usize,
        bank_roll: usize,
        _cards: Vec<Card>,
        _stage: Stage,
        _cycle: u8,
    ) -> Bet {
        if bank_roll == 0 {
            Bet::Fold
        } else if bank_roll <= call {
            Bet::AllIn(bank_roll)
        } else if call == 0 {
            Bet::Raise(min)
        } else {
            Bet::Call
        }
    }

    #[test]
    fn test_place_bets_modest_strategy() {
        let mut game = Game::build(20, 2);
        let _ = game.join(AutoPlayer::build("player1"));
        let _ = game.join(AutoPlayer::build("player2"));
        game.order_players();
        game.deal_hole_cards();

        // Test with a player who bets once.
        let p2 = game.players.get_mut("player2").unwrap();

        p2.set_betting_strategy(test_strategy);
        game.place_bets();
        assert!(
            game.pot == 40,
            "Expected game.pot to be 40, was {}",
            game.pot
        );
        game.players.iter().for_each(|(_name, p)| {
            assert!(
                p.get_bank_roll() == 1980,
                "Expected p.bank_roll to be 1980, was {}.",
                p.get_bank_roll()
            );
        });
    }

    #[test]
    fn test_place_bets_folded_player() {
        let mut game = Game::build(20, 3);
        let _ = game.join(AutoPlayer::build("player1"));
        let _ = game.join(AutoPlayer::build("player2"));
        let _ = game.join(AutoPlayer::build("player3"));
        game.order_players();
        game.deal_hole_cards();

        // Test with a player who bets once and one which is folded.

        let p2 = game.players.get_mut("player2").unwrap();
        p2.set_folded(true);

        let p3 = game.players.get_mut("player3").unwrap();
        p3.set_betting_strategy(test_strategy);

        game.place_bets();
        assert!(
            game.pot == 40,
            "Expected game.pot to be 40, was {}",
            game.pot
        );
        game.players.iter().for_each(|(_name, p)| {
            if p.name == "player2" {
                assert!(
                    p.get_bank_roll() == 2000,
                    "Expected p.bank_roll to be 2000, was {}.",
                    p.get_bank_roll()
                );
            } else {
                assert!(
                    p.get_bank_roll() == 1980,
                    "Expected p.bank_roll to be 1980, was {}.",
                    p.get_bank_roll()
                );
            }
        });
    }

    #[test]
    fn test_deal_flop() {
        let mut game = Game::build(20, 2);
        let _ = game.join(AutoPlayer::build("player1"));
        let _ = game.join(AutoPlayer::build("player2"));
        game.deal_hole_cards();
        game.place_bets();
        game.deal_flop();
        assert!(
            game.deck.len() == 44,
            "Expected game.deck.len() to be 44, was {}",
            game.deck.len()
        );
        assert!(
            game.community_cards.len() == 3,
            "Expected 3 community cards, was {}",
            game.community_cards.len()
        );
        game.community_cards.iter().for_each(|c: &Card| {
            assert!(
                !game.deck.contains(c),
                "Expected {:?} not to be in game.deck but it was.",
                c
            );
        });
    }

    #[test]
    fn test_deal_turn() {
        let mut game = Game::build(20, 2);
        let _ = game.join(AutoPlayer::build("player1"));
        let _ = game.join(AutoPlayer::build("player2"));
        game.deal_hole_cards();
        game.place_bets();
        game.deal_flop();
        game.place_bets();
        game.deal_turn();
        assert!(
            game.deck.len() == 42,
            "Expected game.deck.len() to be 42, was {}",
            game.deck.len()
        );
        assert!(
            game.community_cards.len() == 4,
            "Expected 4 community cards, was {}",
            game.community_cards.len()
        );
        game.community_cards.iter().for_each(|c: &Card| {
            assert!(
                !game.deck.contains(c),
                "Expected {:?} not to be in game.deck but it was.",
                c
            );
        });
    }

    #[test]
    fn test_deal_river() {
        let mut game = Game::build(20, 2);
        let _ = game.join(AutoPlayer::build("player1"));
        let _ = game.join(AutoPlayer::build("player2"));
        game.deal_hole_cards();
        game.place_bets();
        game.deal_flop();
        game.place_bets();
        game.deal_turn();
        game.place_bets();
        game.deal_river();
        assert!(
            game.deck.len() == 40,
            "Expected game.deck.len() to be 40, was {}",
            game.deck.len()
        );
        assert!(
            game.community_cards.len() == 5,
            "Expected 5 community cards, was {}",
            game.community_cards.len()
        );
        game.community_cards.iter().for_each(|c: &Card| {
            assert!(
                !game.deck.contains(c),
                "Expected {:?} not to be in game.deck but it was.",
                c
            );
        });
    }

    #[test]
    fn test_showdown() {
        let mut game = Game::build(20, 2);
        let _ = game.join(AutoPlayer::build("player1"));
        let _ = game.join(AutoPlayer::build("player2"));

        // test outight winner
        let p1_hole = Some((
            Card {
                rank: Rank::Rank10,
                suit: Suit::Clubs,
            },
            Card {
                rank: Rank::Rank4,
                suit: Suit::Clubs,
            },
        ));

        let p2_hole = Some((
            Card {
                rank: Rank::Rank8,
                suit: Suit::Clubs,
            },
            Card {
                rank: Rank::Rank4,
                suit: Suit::Hearts,
            },
        ));
        game.players.iter_mut().for_each(|(name, p)| {
            if name == "player1" {
                p.accept_hole_cards(p1_hole);
            } else {
                p.accept_hole_cards(p2_hole);
            }
        });

        game.community_cards = vec![
            Card {
                rank: Rank::Rank2,
                suit: Suit::Clubs,
            },
            Card {
                rank: Rank::Jack,
                suit: Suit::Clubs,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Clubs,
            },
            Card {
                rank: Rank::Rank4,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Rank5,
                suit: Suit::Hearts,
            },
        ];

        game.showdown();

        let w = game.winner.clone();

        if let Some(Winner::Winner {
            name: n,
            hand: h,
            cards: _cs,
        }) = w
        {
            assert!(n == "player1", "Expected player1, was {}", n);
            assert!(
                h == Hand::Flush(
                    Rank::Rank2,
                    Rank::Rank4,
                    Rank::Rank10,
                    Rank::Jack,
                    Rank::King
                ),
                "Expected Flush(K), was {:?}",
                h
            );
        } else {
            panic!("Expected a winner.");
        }
        // test a draw

        let p1_hole = Some((
            Card {
                rank: Rank::Rank10,
                suit: Suit::Clubs,
            },
            Card {
                rank: Rank::Rank4,
                suit: Suit::Diamonds,
            },
        ));

        let p2_hole = Some((
            Card {
                rank: Rank::Rank10,
                suit: Suit::Spades,
            },
            Card {
                rank: Rank::Rank4,
                suit: Suit::Clubs,
            },
        ));

        game.players.iter_mut().for_each(|(name, p)| {
            if name == "player1" {
                p.accept_hole_cards(p1_hole);
            } else {
                p.accept_hole_cards(p2_hole);
            }
        });

        game.community_cards = vec![
            Card {
                rank: Rank::Rank10,
                suit: Suit::Hearts,
            },
            Card {
                rank: Rank::Rank8,
                suit: Suit::Spades,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Rank3,
                suit: Suit::Hearts,
            },
            Card {
                rank: Rank::Rank2,
                suit: Suit::Hearts,
            },
        ];

        game.showdown();

        let w = game.winner.clone();

        if let Some(Winner::Draw(winners)) = w {
            assert!(
                winners.len() == 2,
                "Expected 2 winners, was {}",
                winners.len()
            );
            winners.iter().for_each(
                |PlayerHand {
                     name: _name,
                     best_hand: h,
                     cards: _cs,
                 }| {
                    assert!(
                        h == &Hand::OnePair(Rank::Rank10),
                        "Expected player to have OnePair(10), was {:?}.",
                        h
                    );
                },
            );
        } else {
            panic!("Expected a draw.");
        }
    }
    #[test]
    fn test_distribute_pot() {
        let mut game = Game::build(20, 3);
        let _ = game.join(AutoPlayer::build("player1"));
        let _ = game.join(AutoPlayer::build("player2"));

        game.deal_hole_cards();
        // test outight winner
        game.pot = 120;
        game.winner = Some(Winner::Winner {
            name: "player1".to_string(),
            hand: Hand::HighCard(Rank::Ace),
            cards: Vec::new(),
        });

        game.distribute_pots();

        assert!(game.pot == 0, "Expected game.pot == 0, was {}", game.pot);
        assert!(
            game.side_pots.is_empty(),
            "Expected no side pots, was {:?}",
            game.side_pots
        );

        let w = game.winner.clone();

        if let Some(Winner::Winner {
            name,
            hand: _hand,
            cards: _cards,
        }) = w
        {
            let p = game.players.get(&name).unwrap();
            assert!(
                p.get_bank_roll() == 2120,
                "Expected winner bankroll to be 2120, was {}",
                p.get_bank_roll()
            );
        } else {
            panic!("Expected a winner.");
        }

        // test a draw with no side pot

        game.players.iter_mut().for_each(|(_name, p)| {
            p.set_bank_roll(0);
        });
        game.pot = 120;
        game.winner = Some(Winner::Draw(vec![
            PlayerHand {
                name: "player1".to_string(),
                best_hand: Hand::HighCard(Rank::Ace),
                cards: Vec::new(),
            },
            PlayerHand {
                name: "player2".to_string(),
                best_hand: Hand::HighCard(Rank::Ace),
                cards: Vec::new(),
            },
        ]));

        game.distribute_pots();

        assert!(game.pot == 0, "Expected game.pot == 0, was {}", game.pot);

        let w = game.winner.clone();

        if let Some(Winner::Draw(winners)) = w {
            winners.iter().for_each(
                |PlayerHand {
                     name,
                     best_hand: _h,
                     cards: _cs,
                 }| {
                    let p = game.players.get(name).unwrap();
                    assert!(
                        p.get_bank_roll() == 60,
                        "Expected player to have bankroll == 60, was {}.",
                        p.get_bank_roll()
                    );
                },
            );
        } else {
            panic!("Expected a draw.");
        }

        // test a draw with a side pot

        let _ = game.join(AutoPlayer::build("player3"));
        //game.deal_hole_cards();
        // players 2 and 3 are all in
        game.players.iter_mut().for_each(|(_name, p)| {
            p.set_bank_roll(0);
            p.accept_hole_cards(Some((
                Card {
                    rank: Rank::Rank2,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::Rank3,
                    suit: Suit::Clubs,
                },
            )));
            if p.get_name() == "player2" || p.get_name() == "player3" {
                p.set_all_in(true);
            }
        });
        // main pot should be 120 /3 = 40 for each player
        game.pot = 120;
        // side pot of 60 chips goes to players 1 and 3, 30 each
        game.side_pots = vec![SidePot {
            players: vec!["player1".to_string(), "player3".to_string()],
            pot: 60,
        }];
        game.winner = Some(Winner::Draw(vec![
            PlayerHand {
                name: "player1".to_string(),
                best_hand: Hand::HighCard(Rank::Ace),
                cards: Vec::new(),
            },
            PlayerHand {
                name: "player2".to_string(),
                best_hand: Hand::HighCard(Rank::Ace),
                cards: Vec::new(),
            },
            PlayerHand {
                name: "player3".to_string(),
                best_hand: Hand::HighCard(Rank::Ace),
                cards: Vec::new(),
            },
        ]));

        game.distribute_pots();

        assert!(game.pot == 0, "Expected game.pot == 0, was {}", game.pot);

        let w = game.winner.clone();

        if let Some(Winner::Draw(winners)) = w {
            winners.iter().for_each(
                |PlayerHand {
                     name,
                     best_hand: _h,
                     cards: _cs,
                 }| {
                    let p = game.players.get(name).unwrap();
                    if p.get_name() == "player1" || p.get_name() == "player3" {
                        assert!(
                            p.get_bank_roll() == 70,
                            "Expected non-all inplayer to split main pot (120/3) and side pot (60/2) = 70, was {}.",
                            p.get_bank_roll()
                        );
                    } else {
                        assert!(
                            p.get_bank_roll() == 40,
                            "Expected all in player to split main pot (120/3) = 40, was {}.",
                            p.get_bank_roll()
                        );
                    }
                },
            );
        } else {
            panic!("Expected a draw.");
        }
    }
}
