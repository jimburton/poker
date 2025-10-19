use crate::poker::card::{new_deck, Card, Hand};
use crate::poker::compare::{best_hand, compare_hands};
use crate::poker::player::Player;
use crate::poker::rotate_vector;
use num_traits::ToPrimitive;
use rand::rng;
use rand::seq::SliceRandom;
use std::collections::HashMap;

#[derive(Debug, Clone)]
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
    Fold,
    Check,
    Call(usize),
    Raise(usize),
    AllIn(usize),
}

pub fn new_game(buy_in: usize, small_blind: usize, big_blind: usize, num_players: u8) -> Game {
    Game::build(buy_in, small_blind, big_blind, num_players)
}

#[derive(Debug)]
pub struct Game {
    players: HashMap<String, Player>,
    players_order: Vec<String>,
    dealer: Option<String>,
    current_player: Option<Player>,
    buy_in: usize,
    small_blind: usize,
    big_blind: usize,
    call: usize,
    pot: usize,
    side_pots: Vec<SidePot>,
    deck: Vec<Card>,
    community_cards: Vec<Card>,
    num_players: u8,
    side_pot_active: bool,
    winner: Option<Winner>,
    first_round: bool,
}

#[derive(Debug, Clone)]
struct SidePot {
    players: Vec<String>,
    pot: usize,
}

impl Game {
    pub fn build(buy_in: usize, small_blind: usize, big_blind: usize, num_players: u8) -> Self {
        let mut game = Game {
            players: HashMap::new(),
            players_order: Vec::new(),
            dealer: None,
            current_player: None,
            buy_in,
            small_blind,
            big_blind,
            call: 0,
            pot: 0,
            side_pots: Vec::new(),
            deck: Vec::new(),
            community_cards: Vec::new(),
            num_players,
            side_pot_active: false,
            winner: None,
            first_round: true,
        };
        let mut deck = new_deck();
        let mut rng = rng();
        deck.shuffle(&mut rng);
        game.deck = deck;

        game
    }

    fn full(&self) -> bool {
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
        self.players_order.push(name.to_string());
        Ok(())
    }

    /// Play a game.
    pub fn play(&mut self) {
        self.play_round();
        // TODO play another, until WHAT?
    }

    /// Set the name of the dealer and re order the players_order list
    /// so that the player to the left of the dealer is at the front.
    fn order_players(&mut self) {
        if self.first_round {
            let players_order: Vec<String> = self.players_order.clone();
            self.dealer = Some(players_order.first().unwrap().clone());
            self.first_round = false;
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
            panic!("Can't find player with name {}", dealer.unwrap());
        }
    }

    /// Play a round.
    fn play_round(&mut self) {
        self.order_players();
        self.ante_up();
        self.deal_hole_cards();
        self.place_bets();
        self.deal_flop();
        self.place_bets();
        self.deal_turn();
        self.place_bets();
        self.deal_river();
        self.place_bets();
        self.showdown();
        self.distribute_pots();
    }

    /// Each player buys in once after joining, or is removed from the game.
    fn player_buy_in(&mut self, name: &str) {
        let mut not_joining = false;
        if let Some(p) = self.players.get_mut(name) {
            if let Ok(n) = p.ante_up(self.buy_in) {
                self.pot += n
            } else {
                not_joining = true;
            }
        }
        if not_joining {
            let _ = self.players.remove(name);
        }
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
                // NB: player marks themself as folded if they responded negatively.
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
            p.hole = Some((c1, c2));
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

    /// Players are given the opportunity to bet at the following stages:
    /// + after Hole cards are dealt,
    /// + after the Flop is dealt,
    /// + after the Turn is dealt,
    /// + after the River is dealt.
    ///
    /// The player immediately to the left of the dealer button is
    /// the *smallblind* and is the first player to act. Their minimum bet is
    /// equal to the small blind. Betting then proceeds clockwise, with the
    /// minimum bet for everyone else being the big blind.
    ///
    /// On a players turn to bet, then if no bet has been made they can:
    /// + check -- pass the action without contributing to the pot,
    /// + fold -- retire from this round,
    /// + bet -- make a bet which is the minimum amount or more.
    ///
    /// If a bet has been made in this round then they must
    /// + fold,
    /// + call -- pay the amount of the current bet,
    /// + raise -- pay an amount greater than the current bet.
    ///
    /// When a player raises the bet everyone else has to call, fold or raise,
    /// so this can go round in a circle until everyone is all in.
    ///
    /// If a player goes "all in" by betting their entire bank roll, a side
    /// pot becomes active and subsequent bets are paid into it. Each time a
    /// player goes all in, a new side pot is created.
    ///
    /// If an all-in player wins the round, they win the main pot plus any side pots
    /// they contributed to (or a share thereof if there is a draw). If side pots
    /// exist which the winner did not contribute to, the money in that side pot
    /// is distributed to the player(s) with the best hands mongst those who
    /// contributed to it and are still in the game.
    ///   
    ///
    /// If, at the end of a round, a side pot exists where none of the players
    /// who contributed to it are still in the game, that side pot is distributed
    /// among the winners of the main pot.
    ///
    /// TODO not clear if I've got the rules about who bets first right, some
    /// sources suggest it should be the *big blind*, which is the player to the
    /// left of the small blind.
    fn place_bets(&mut self) {
        let players_order = self.players_order.clone();
        let players = self.players.clone();
        if players_order.is_empty() {
            return;
        }
        let mut not_all_in = Vec::new();
        {
            for p in players.values() {
                if !p.all_in {
                    not_all_in.push(&p.name);
                }
            }
        }
        let mut current: usize = 0;
        let mut target: &String = self.players_order.last().unwrap();
        let mut done: bool = false;
        let mut call: usize = 0;
        let min = self.big_blind;
        while !done {
            let current_name = self.players_order[current];
            let p = self.players.get_mut(&current_name).unwrap();
            if &p.name == target {
                done = true;
            } else {
                if !p.folded {
                    let ccards = self.community_cards.clone();
                    let bet = p.place_bet(call, min, ccards);
                    match bet {
                        Bet::Fold => {}
                        Bet::Check => {}
                        Bet::Call(n) => {
                            if call == n {
                                self.pot += call;
                            } else {
                                dbg!("Tried to call with amount too small.");
                                p.folded = true;
                            }
                        }
                        Bet::Raise(raise) => {
                            if raise > call {
                                if self.side_pot_active {
                                    let mut side_pot = self.side_pots.get(0).unwrap();
                                    side_pot.pot += raise;
                                } else {
                                    self.pot += raise;
                                }
                                call += raise - call;
                                target = &p.name;
                            } else {
                                dbg!("Tried to raise less than call.");
                                p.folded = true;
                            }
                        }
                        Bet::AllIn(n) => {
                            self.pot += n;
                            self.side_pot_active = true;
                            if let Some(index) =
                                not_all_in.iter().position(|value| **value == p.name)
                            {
                                not_all_in.swap_remove(index);
                            }
                            let not_all_in = not_all_in.clone().iter().map(|n| **n).collect();
                            let new_side_pot = SidePot {
                                players: not_all_in,
                                pot: 0,
                            };
                            self.side_pots.push(new_side_pot);
                        }
                    }
                }
                current = current + 1 % self.players_order.len();
            }
        }
    }

    /// Determines the winner(s) of the round.
    pub fn showdown(&mut self) {
        // Calculate the best hand for each non-folded player.
        let mut hands: Vec<(String, Hand, Vec<Card>)> =
            self.names_to_hands(self.players.keys().cloned().collect());
        // Handle cases where 0 or 1 players remain (the last player standing wins)
        if hands.len() < 2 {
            if let Some((name, hand, cards)) = hands.pop() {
                let winner = Winner::Winner { name, hand, cards };
                dbg!("Winner (last player standing): {:?}", &winner);
                self.winner = Some(winner);
            } else {
                dbg!("No players remaining to determine winner.");
            }
            return;
        }

        let winner = Game::determine_winner(hands);
        dbg!("Final Showdown Result: {:?}", &winner);
        self.winner = Some(winner);
    }

    /// Take a vector of player names and return a vector of (name, hand, cards),
    /// where hand is their best hand and cards is their hole cards plus community cards.
    /// Result contains only non-folded players.
    fn names_to_hands(&self, names: Vec<String>) -> Vec<(String, Hand, Vec<Card>)> {
        // Calculate the best hand for each non-folded player.
        let hands: Vec<(String, Hand, Vec<Card>)> = self
            .players
            .iter() // Use iter() since we don't need to mutate Player state here
            .filter_map(|(_, p)| {
                // Only consider players who haven't folded
                if p.folded || !names.contains(&p.name) {
                    return None;
                }

                let (c1, c2) = p
                    .hole
                    .expect("Hole cards should be dealt before calling names_to_hands");

                // Collect all 7 cards (2 hole + 5 community)
                let mut all_cards = self.community_cards.clone();
                all_cards.push(c1);
                all_cards.push(c2);

                let best_hand = best_hand(&all_cards);
                Some((p.name.clone(), best_hand, all_cards))
            })
            .collect();
        hands
    }

    /// Determine winner(s) from vector of (name, best_hand, cards) tuples.
    fn determine_winner(mut hands: Vec<(String, Hand, Vec<Card>)>) -> Winner {
        // Handle cases where 0 or 1 players remain (the last player standing wins)
        if hands.len() < 2 {
            if let Some((name, hand, cards)) = hands.pop() {
                dbg!("determine_winner: last player standing: {}", name.clone());
                Winner::Winner { name, hand, cards };
            } else {
                dbg!("No players remaining to determine winner.");
            }
        }

        // Compare the hands.

        // Initialize winner with the first player's hand, consuming it from the vector.
        let mut winner: Winner = {
            let (name, hand, cards) = hands.remove(0);
            Winner::Winner { name, hand, cards }
        };

        // Compare current winner against all remaining hands.
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
                        // Challenger data is moved here
                        (challenger_name, challenger_hand, challenger_cards),
                        (w_name, w_hand, w_cards),
                    )
                }
                Winner::Draw(mut draw_winners) => {
                    // If it's a draw, compare the challenger against the best hand in the draw group.
                    // Assume the first element of a Draw is the benchmark.
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
                            // Challenger ties with the benchmark, add challenger to the draw group.
                            // The original (un-cloned) challenger values are now moved here.
                            draw_winners.push((challenger_name, challenger_hand, challenger_cards));
                            Winner::Draw(draw_winners)
                        }
                    }
                }
            };
        }
        winner
    }

    /// Determine the winner of the side pot.
    fn determine_side_pot_winner(&self, sp: &SidePot) -> Winner {
        let hands = self.names_to_hands(sp.players.clone());
        Game::determine_winner(hands)
    }

    /// Allocate side pot winnings. Note that we don't check whether the players in the
    /// side pot are folded or not, this should be done by the caller.
    fn allocate_side_pot_winnings(&mut self, w: &Winner, sp: &SidePot) {
        match w {
            Winner::Winner { name, .. } => {
                self.players.get_mut(name).unwrap().bank_roll += sp.pot;
            }
            Winner::Draw(players) => {
                players.iter().for_each(|(name, _hand, _cards)| {
                    let win = sp.pot / sp.players.len();
                    dbg!("{} wins {} from side pot", name, win);
                    self.players.get_mut(name).unwrap().bank_roll += win;
                });
            }
        }
    }

    /// Filter a list of player names to those which aren't currently folded.
    fn not_folded(&self, players: &Vec<String>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let players = players.clone();
        for name in players {
            if self.players.contains_key(&name) && !self.players.get(&name).unwrap().folded {
                result.push(name);
            }
        }
        result
    }

    /// Distributes the pot and side pot to the winner(s).
    ///
    /// TODO keep track of chips being lost due to truncating division.
    fn distribute_pots(&mut self) {
        if let Some(winner) = &self.winner {
            match winner {
                Winner::Winner { name, .. } => {
                    dbg!("\n--- Pot Distribution (outright winner) ---");
                    if let Some(player) = self.players.get_mut(name) {
                        if !player.all_in {
                            let side_pot: usize = self.side_pots.iter().map(|sp| sp.pot).sum();
                            let total_pot: usize = self.pot + side_pot;
                            player.bank_roll += total_pot;
                            dbg!("{} wins {} chips!", name, total_pot);
                        } else {
                            // Deal with side pots.
                            // FIX: Clone side_pots to iterate over owned data, releasing the immutable borrow on `self`.
                            let side_pots_to_process = self.side_pots.clone();
                            let mut winner_side_pot: usize = 0;

                            // FIX: Switched from for_each to a standard 'for' loop for safer mutable access.
                            for mut sp in side_pots_to_process {
                                // if the winner is in this side pot they get the winnings.
                                if sp.players.contains(name) {
                                    winner_side_pot += sp.pot;
                                } else {
                                    // determine winners and allocate winnings.
                                    let w = self.determine_side_pot_winner(&sp);
                                    // players in side pot who did not fold
                                    let players = sp.players.clone();
                                    let active = self.not_folded(&&players);
                                    if !active.is_empty() {
                                        // there are some left, they get the side pot
                                        sp.players = active;
                                        // FIX: Replace self.allocate_side_pot_winnings with inline allocation logic
                                        // to separate the immutable borrow (from w) from the mutable borrow (self.players.get_mut).
                                        match w {
                                            Winner::Winner {
                                                name: winner_name, ..
                                            } => {
                                                self.players
                                                    .get_mut(&winner_name)
                                                    .unwrap()
                                                    .bank_roll += sp.pot;
                                            }
                                            Winner::Draw(draw_players) => {
                                                let win = sp.pot / draw_players.len();
                                                draw_players.iter().for_each(
                                                    |(name, _hand, _cards)| {
                                                        dbg!("{} wins {} from side pot", name, win);
                                                        self.players
                                                            .get_mut(name)
                                                            .unwrap()
                                                            .bank_roll += win;
                                                    },
                                                );
                                            }
                                        }
                                    } else {
                                        // side pot goes to winner even though they weren't in it
                                        winner_side_pot += sp.pot;
                                    }
                                }
                            }
                            player.bank_roll += winner_side_pot;
                            dbg!("{} wins {} chips!", name, winner_side_pot);
                        }
                    } else {
                        dbg!("Error: Winner {} not found in player list.", name);
                    }
                }
                Winner::Draw(draw_winners) => {
                    // no outright winner
                    let num_winners = draw_winners.len();
                    let num_winners_not_all_in = draw_winners
                        .iter()
                        .filter(|(name, _hand, _cards)| {
                            dbg!("Looking for player: {}", name);
                            let p = self.players.get(name).expect("Couldn't find player");
                            !p.all_in
                        })
                        .collect::<Vec<_>>()
                        .len();

                    if num_winners == 0 {
                        dbg!("Error: Draw with no winners found.");
                        return;
                    }
                    // the winners take an equal share of the main pot
                    let main_pot_share = self.pot / num_winners;
                    dbg!("\n--- Pot Distribution (draw) ---");
                    dbg!(
                        "Draw between {} players. Each player gets {} share of the main pot.",
                        num_winners,
                        main_pot_share
                    );

                    let mut winners_pot_share: usize = main_pot_share;

                    // Deal with side pots.
                    // We clone the side pots here to safely iterate over them while
                    // potentially modifying 'self' (e.g., player bank_roll) inside the loop.
                    let side_pots_to_process = self.side_pots.clone();

                    let mut unclaimed_side_pots: usize = 0;

                    for mut sp in side_pots_to_process {
                        // Iterate over owned data
                        // determine winners and allocate winnings.
                        let w = self.determine_side_pot_winner(&sp);
                        // players in side pot who did not fold
                        // Using sp.players directly now that sp is mut and owned
                        let active = self.not_folded(&sp.players);
                        if !active.is_empty() {
                            // there are some left, they get the side pot
                            sp.players = active;
                            match w {
                                Winner::Winner { name, .. } => {
                                    self.players.get_mut(&name).unwrap().bank_roll += sp.pot;
                                }
                                Winner::Draw(players) => {
                                    // Fix: Use 'players.len()' (number of winners) for division
                                    let win = sp.pot / players.len();
                                    players.iter().for_each(|(name, _hand, _cards)| {
                                        dbg!("{} wins {} from side pot", name, win);
                                        self.players.get_mut(name).unwrap().bank_roll += win;
                                    });
                                }
                            }
                        } else {
                            // side pot goes to winner even though they weren't in it
                            unclaimed_side_pots += sp.pot;
                        }
                    }
                    winners_pot_share += unclaimed_side_pots / draw_winners.len();
                    dbg!(
                        "Each winner gets an additional {} from side pots.",
                        winners_pot_share
                    );
                    for (name, _, _) in draw_winners.into_iter() {
                        if let Some(player) = self.players.get_mut(name) {
                            player.bank_roll += winners_pot_share;
                        } else {
                            panic!("Couldn't find player.");
                        }
                    }
                }
            }
        } else {
            dbg!("Winner not chosen.");
        }
    }

    fn reset_after_round(&mut self) {
        self.pot = 0;
        self.side_pots = Vec::new();
        dbg!("Pots reset.");

        let dealer_name_ref: Option<&String> = self.dealer.as_ref();

        let mut removed_names: Vec<String> = Vec::new();

        // Loop through the players resetting all_in and folded, and collecting
        // the list of ones that need to be removed for lack of money.
        for name in self.players_order.iter() {
            let p = self
                .players
                .get_mut(name)
                .expect("Player in order list not found in map.");
            p.all_in = false;
            p.folded = false;
            let is_dealer = dealer_name_ref.map_or(false, |d_name| d_name == name);
            if is_dealer && p.bank_roll < self.small_blind {
                removed_names.push(name.clone());
            } else if p.bank_roll < self.big_blind {
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
        dbg!("Players that ran out of money removed.");
    }
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
            "Expected game.deck to have 42 cards, was {}",
            game.deck.len()
        );
    }

    #[test]
    fn test_add_player() {
        let mut game = Game::build(10, 2);
        let _ = game.add_player("player1", 100);
        let _ = game.add_player("player2", 100);
        if let Err(e) = game.add_player("player3", 100) {
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
    fn test_players_buy_in() {
        let mut game = Game::build(20, 2);
        let _ = game.add_player("player1", 100);
        let _ = game.add_player("player2", 100);
        game.players_buy_in();
        assert!(
            game.pot == 40,
            "Expected game.pot to be 40, was {}",
            game.pot
        );
        game.players.iter().for_each(|(_name, p)| {
            assert!(
                p.bank_roll == 80,
                "Expected p.bank_roll to be 80, was {}",
                p.bank_roll
            )
        });
    }

    #[test]
    fn test_deal_hole_cards() {
        let mut game = Game::build(20, 2);
        let _ = game.add_player("player1", 100);
        let _ = game.add_player("player2", 100);
        game.players_buy_in();
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
    fn test_place_bets() {
        let mut game = Game::build(20, 2);
        let _ = game.add_player("player1", 100);
        let _ = game.add_player("player2", 100);
        game.players_buy_in();
        game.deal_hole_cards();
        game.place_bets();
        assert!(
            game.pot == 80,
            "Expected game.pot to be 80, was {}",
            game.pot
        );
        game.players.iter().for_each(|(_name, p)| {
            assert!(
                p.bank_roll == 60,
                "Expected p.bank_roll to be 60, was {}.",
                p.bank_roll
            );
        });
    }

    #[test]
    fn test_deal_flop() {
        let mut game = Game::build(20, 2);
        let _ = game.add_player("player1", 100);
        let _ = game.add_player("player2", 100);
        game.players_buy_in();
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
        let _ = game.add_player("player1", 100);
        let _ = game.add_player("player2", 100);
        game.players_buy_in();
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
        let _ = game.add_player("player1", 100);
        let _ = game.add_player("player2", 100);
        game.players_buy_in();
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
        let _ = game.add_player("player1", 100);
        let _ = game.add_player("player2", 100);

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
                p.hole = p1_hole;
            } else {
                p.hole = p2_hole;
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
                p.hole = p1_hole;
            } else {
                p.hole = p2_hole;
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
            winners.iter().for_each(|(_name, h, _cs)| {
                assert!(
                    h == &Hand::OnePair(Rank::Rank10),
                    "Expected player to have OnePair(10), was {:?}.",
                    h
                );
            });
        } else {
            panic!("Expected a draw.");
        }
    }

    #[test]
    fn test_distribute_pot() {
        let mut game = Game::build(20, 3);
        let _ = game.add_player("player1", 0);
        let _ = game.add_player("player2", 0);

        // test outight winner
        game.pot = 120;
        game.winner = Some(Winner::Winner {
            name: "player1".to_string(),
            hand: Hand::HighCard(Rank::Ace),
            cards: Vec::new(),
        });

        game.distribute_pot();

        assert!(game.pot == 0, "Expected game.pot == 0, was {}", game.pot);

        let w = game.winner.clone();

        if let Some(Winner::Winner {
            name,
            hand: _hand,
            cards: _cards,
        }) = w
        {
            let p = game.players.get(&name).unwrap();
            assert!(
                p.bank_roll == 120,
                "Expected winner bankroll to be 120, was {}",
                p.bank_roll
            );
        } else {
            panic!("Expected a winner.");
        }

        // test a draw with no side pot

        game.players.iter_mut().for_each(|(_name, p)| {
            p.bank_roll = 0;
        });
        game.pot = 120;
        game.winner = Some(Winner::Draw(vec![
            ("player1".to_string(), Hand::HighCard(Rank::Ace), Vec::new()),
            ("player2".to_string(), Hand::HighCard(Rank::Ace), Vec::new()),
        ]));

        game.distribute_pot();

        assert!(game.pot == 0, "Expected game.pot == 0, was {}", game.pot);

        let w = game.winner.clone();

        if let Some(Winner::Draw(winners)) = w {
            winners.iter().for_each(|(name, _h, _cs)| {
                let p = game.players.get(name).unwrap();
                assert!(
                    p.bank_roll == 60,
                    "Expected player to have bankroll == 60, was {}.",
                    p.bank_roll
                );
            });
        } else {
            panic!("Expected a draw.");
        }

        // test a draw with a side pot

        let _ = game.add_player("player3", 0);
        game.players.iter_mut().for_each(|(_name, p)| {
            p.bank_roll = 0;
            if p.name == "player2" || p.name == "player3" {
                p.all_in = true;
            }
        });
        game.pot = 120;
        game.side_pot = 60;
        game.winner = Some(Winner::Draw(vec![
            ("player1".to_string(), Hand::HighCard(Rank::Ace), Vec::new()),
            ("player2".to_string(), Hand::HighCard(Rank::Ace), Vec::new()),
            ("player3".to_string(), Hand::HighCard(Rank::Ace), Vec::new()),
        ]));

        game.distribute_pot();

        assert!(game.pot == 0, "Expected game.pot == 0, was {}", game.pot);

        let w = game.winner.clone();

        if let Some(Winner::Draw(winners)) = w {
            winners.iter().for_each(|(name, _h, _cs)| {
                let p = game.players.get(name).unwrap();
                if p.name == "player1" {
                    assert!(
                        p.bank_roll == 100,
                        "Expected non-all inplayer to have bankroll == 100, was {}.",
                        p.bank_roll
                    );
                } else {
                    assert!(
                        p.bank_roll == 40,
                        "Expected all in player to have bankroll == 40, was {}.",
                        p.bank_roll
                    );
                }
            });
        } else {
            panic!("Expected a draw.");
        }
    }
}
