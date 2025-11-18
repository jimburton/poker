/// Datatypes and functions for the game and individual rounds.
use crate::poker::{
    betting_strategy::BetArgs,
    card,
    card::Card,
    compare, names,
    player::{Msg, Player, PlayerHand, Winner},
    rotate_vector,
};
use rand::{rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{self, Display},
};
use uuid::Uuid;

// minimum and maximum number of players in a game.
const MIN_PLAYERS: u8 = 2;
const MAX_PLAYERS: u8 = 6;

/// Enum for representing the stage of a round.
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Stage {
    Blinds,
    Hole,
    PreFlop,
    Flop,
    Turn,
    River,
    ShowDown,
}
/// Implementarion of Display trait for Stage.
impl Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stage::Blinds => write!(f, "Blinds"),
            Stage::Hole => write!(f, "Hole"),
            Stage::PreFlop => write!(f, "Pre-Flop"),
            Stage::Flop => write!(f, "Flop"),
            Stage::Turn => write!(f, "Turn"),
            Stage::River => write!(f, "River"),
            Stage::ShowDown => write!(f, "Showdown"),
        }
    }
}

/// Enum for representing a bet.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Bet {
    Fold,
    Check,
    Call,
    Raise(usize),
    AllIn(usize),
}
/// Implementation of Display trait for Bet.
impl Display for Bet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Bet::Fold => write!(f, "Fold"),
            Bet::Check => write!(f, "Check"),
            Bet::Call => write!(f, "Call"),
            Bet::Raise(amount) => write!(f, "Raise ({})", amount),
            Bet::AllIn(amount) => write!(f, "All in ({})", amount),
        }
    }
}

/// Struct for a side pot.
#[derive(Debug, Clone)]
struct SidePot {
    players: Vec<String>,
    pot: usize,
}

/// Struct for the game.
#[allow(unused)]
#[derive(Debug)]
pub struct Game {
    players: HashMap<String, Box<Player>>,
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
    uuid: uuid::Uuid,
}

/// Implementation for the Game struct.
impl Game {
    pub fn build(big_blind: usize, max_players: u8) -> Self {
        if max_players > MAX_PLAYERS {
            panic!("The maximum number of players is {}", MAX_PLAYERS);
        }
        if max_players < MIN_PLAYERS {
            panic!("The minimum number of players is {}", MIN_PLAYERS);
        }
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
            uuid: Uuid::new_v4(),
        };
        let mut deck = card::new_deck();
        let mut rng = rng();
        deck.shuffle(&mut rng);
        game.deck = deck;

        game
    }

    /// Predicate function for the game having the full amount of players.
    fn full(&self) -> bool {
        self.players.len() == self.max_players as usize
    }

    /// Allows a player to join the game. The new player's bank roll will be equal to the buy in amount.
    /// The player's name may be changed to make it unique among existing players. The player
    /// instance is notified of the name and bank roll via Player::set_name_and_bank_roll.
    pub fn join(&mut self, mut player: Player) -> Result<(), &'static str> {
        if self.full() {
            return Err("Cannot add more players.");
        }
        let name = names::uniquify_name(&player.name, &self.players_order);
        player.set_name_and_bank_roll(&name, self.buy_in);
        self.players.insert(name.clone(), Box::new(player));
        self.players_order.push(name);
        Ok(())
    }

    /// Play a game.
    pub fn play(&mut self) -> Winner {
        while self.players.len() > 1 {
            self.play_round();
            self.reset_after_round();
            self.num_rounds += 1;
        }
        let w = self.get_winner();
        let msg = Msg::GameWinner(w.clone());
        self.update_players(&msg);
        w
    }

    /// Determine the winner at the end of the game. Assumption is that there's only one
    /// player left.
    fn get_winner(&self) -> Winner {
        let winner_opt = self.players_order.first();
        if let Some(name) = winner_opt {
            let winner = self.players.get(name).unwrap();
            let mut cards = self.community_cards.clone();
            let (h1, h2) = winner.hole.unwrap();
            cards.push(h1);
            cards.push(h2);
            let name = winner.name.clone();
            let hand = compare::best_hand(&cards);
            Winner::SoleWinner(PlayerHand { name, hand, cards })
        } else {
            panic!("Announcing winner but they have been removed...")
        }
    }

    /// Announce the players at the beginning of a round.
    fn announce_players(&self) {
        let players = self
            .players_order
            .iter()
            .map(|name| (name.clone(), self.players.get(name).unwrap().bank_roll))
            .collect();
        let dealer = self.dealer.as_ref().unwrap().clone();
        let msg = Msg::PlayersInfo { players, dealer };
        self.update_players(&msg);
    }

    /// Announce the winner at the end of the round.
    fn announce_winner_round(&self) {
        let w = self.winner.as_ref().unwrap();
        let msg = Msg::RoundWinner(w.clone());
        self.update_players(&msg);
    }

    /// Set the name of the dealer and reorder the players_order list
    /// so that the player to the left of the dealer is at the front.
    fn order_players(&mut self) {
        if self.stage == Stage::Blinds {
            let players_order: Vec<String> = self.players_order.clone();
            self.dealer = Some(players_order.first().unwrap().clone());
        }
        let dealer = self.dealer.as_ref();
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
        self.announce_players();
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
        // announce the winner.
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
        {
            let left_of_dealer: &String = players_order.first().unwrap();

            // Mutably borrow the player and execute the action
            if let Some(first_p) = self.players.get_mut(left_of_dealer)
                && let Some(blind) = first_p.ante_up(self.small_blind)
            {
                self.pot += blind;
            }
            // NB: player marks themself as folded if they responded negatively
            // or as all in if their bank roll was less than the blind.
        }

        // Handle remaining players.
        players_order[1..].iter().for_each(|name| {
            if let Some(p) = self.players.get_mut(name)
                && let Some(blind) = p.ante_up(self.big_blind)
            {
                self.pot += blind
            }
        });
    }

    /// Take num cards from the deck.
    fn take_cards(&mut self, num: usize) -> Result<Vec<Card>, &'static str> {
        if self.deck.len() < num {
            Err("Not enough cards left")
        } else {
            let cards: Vec<Card> = self.deck[0..num].to_vec();
            self.deck = self.deck[num..].to_vec();
            Ok(cards)
        }
    }

    /// Burn a card.
    fn burn_card(&mut self) -> Result<(), &'static str> {
        if self.deck.is_empty() {
            Err("No cards left")
        } else {
            self.deck.pop();
            Ok(())
        }
    }

    /// Deal two hole cards to each player.
    fn deal_hole_cards(&mut self) {
        let mut hole_cards = self.take_cards(2 * self.players.len()).unwrap();
        self.players.iter_mut().for_each(|(_, p)| {
            let hole_1 = hole_cards.pop().unwrap();
            let hole_2 = hole_cards.pop().unwrap();
            p.hole_cards((hole_1, hole_2));
        });
    }

    /// Burn one card and deal the first three three community cards.
    fn deal_flop(&mut self) {
        let _burn = self.burn_card();
        let mut flop_cards: Vec<Card> = self.take_cards(3).unwrap();
        self.community_cards.append(flop_cards.as_mut());
    }

    /// Burn one card and deal the fourth community card.
    fn deal_turn(&mut self) {
        let _burn = self.burn_card();
        let mut turn_card: Vec<Card> = self.take_cards(1).unwrap();
        self.community_cards.append(turn_card.as_mut());
    }

    /// Burn one card and deal the fifth and final community card.
    fn deal_river(&mut self) {
        let _burn = self.burn_card();
        let mut river_card: Vec<Card> = self.take_cards(1).unwrap();
        self.community_cards.append(river_card.as_mut());
    }

    /// Players are given the opportunity to bet. If a player raises the bet, every
    /// other player must respond (fold, call or raise again).
    fn place_bets(&mut self) {
        // names of players who have not folded
        let not_folded: Vec<(String, bool)> = self
            .players
            .values()
            .filter(|p| !p.folded)
            .map(|p| (p.name.clone(), p.all_in))
            .collect();
        // names of players who have not folded and are not all in. These are the players who need to make a bet/call/fold.
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

        let update = Msg::StageDeclare(self.stage, self.community_cards.clone());
        self.update_players(&update);

        let mut current_index: usize = 0;
        // target is the player at which the round of betting should stop.
        // This is altered when a player raises the bet.
        let mut target: String = players.first().unwrap().clone();
        let mut done: bool = false; // flag to stop the loop
        let mut target_placed_bet: bool = false; // flag to allow target to place first bet.
        let mut call: usize = 0;
        let min = self.big_blind;
        let mut cycle: u8 = 0; // the number of times players have been given a chance to bet in this round.

        // Ask each player to place a bet at least once. Note that the Player struct is responsible
        // for managing its own state during betting, e.g. keeping the bank roll up to date
        // and whether the player is folded or all in.
        while !done && players.len() > 1 {
            let current_name = &players[current_index % players.len()];
            let p = self.players.get_mut(current_name).unwrap();
            if p.name == target && target_placed_bet {
                done = true;
            } else {
                if p.name == target {
                    target_placed_bet = true;
                }
                if !p.all_in && !p.folded {
                    let ccards = self.community_cards.clone();
                    let args = BetArgs {
                        call,
                        min,
                        stage: self.stage,
                        cycle,
                        community_cards: ccards,
                    };
                    let bet_opt = p.place_bet(args);

                    let bet = bet_opt.unwrap();

                    match bet {
                        Bet::Fold => {
                            players.remove(current_index);
                            continue; // continue without incrementing current
                        }
                        Bet::Check => {
                            if call > 0 {
                                panic!(
                                    "Misbehaving client checked when there was an outstanding bet."
                                );
                            }
                        }
                        Bet::Call => {
                            self.pot += call;
                        }
                        Bet::Raise(raise) => {
                            cycle += 1;
                            if !self.side_pots.is_empty() {
                                let side_pot = self.side_pots.get_mut(0).unwrap();
                                side_pot.pot += raise;
                            } else {
                                self.pot += raise;
                            }
                            // raise is the new amount to match/beat
                            call = raise;
                            target = p.name.clone();
                        }
                        Bet::AllIn(bet) => {
                            self.pot += bet;

                            if let Some(index) =
                                not_all_in.iter().position(|value| value == &p.name)
                            {
                                not_all_in.swap_remove(index);
                            }

                            let new_side_pot = SidePot {
                                players: not_all_in.clone(),
                                pot: 0,
                            };
                            self.side_pots.push(new_side_pot);
                            // don't ask this player again in this round.
                            players.remove(current_index);
                            continue; // continue without incrementing current
                        }
                    }
                    let update = Msg::Bet {
                        player: p.name.clone(),
                        bet,
                        pot: self.pot,
                    };
                    self.update_players(&update);
                }
                current_index = (current_index + 1) % players.len();
            }
        }
    }

    /// Send a message to the players.
    fn update_players(&self, update: &Msg) {
        self.players.values().for_each(|p| {
            p.update(update);
        });
    }

    /// Determines the winner(s) of the round.
    fn showdown(&mut self) {
        // Get the best hand for each non-folded player.
        let mut hands: Vec<PlayerHand> = self.names_to_hands(&self.players_order);
        // Handle cases where 0 or 1 players remain (the last player standing wins)
        if hands.len() < 2 {
            if let Some(PlayerHand {
                name,
                hand: best_hand,
                cards,
            }) = hands.pop()
            {
                let winner = Winner::SoleWinner(PlayerHand {
                    name,
                    hand: best_hand,
                    cards,
                });
                self.winner = Some(winner);
            } else {
                panic!("No players remaining to determine winner.");
            }
            return;
        }

        let winner = Game::determine_winner(hands);
        self.winner = Some(winner);
    }

    /// Takes a vector of player names and return a vector of PlayerHand objects.
    /// Result contains only non-folded players.
    fn names_to_hands(&self, names: &[String]) -> Vec<PlayerHand> {
        // Calculate the best hand for each non-folded player.
        let hands: Vec<PlayerHand> = self
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

                let best_hand = compare::best_hand(&all_cards);
                Some(PlayerHand {
                    name: p.name.clone(),
                    hand: best_hand,
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
                hand: best_hand,
                cards,
            }) = hands.pop()
            {
                return Winner::SoleWinner(PlayerHand {
                    name,
                    hand: best_hand,
                    cards,
                });
            } else {
                panic!("No players remaining to determine winner.");
            }
        }

        // Compare the hands.

        // Initialize winner with the first player's hand.
        let mut winner: Winner = {
            let PlayerHand {
                name,
                hand: best_hand,
                cards,
            } = hands.remove(0);
            Winner::SoleWinner(PlayerHand {
                name,
                hand: best_hand,
                cards,
            })
        };

        // Compare current winner against all remaining hands.
        for PlayerHand {
            name: challenger_name,
            hand: challenger_hand,
            cards: challenger_cards,
        } in hands
        {
            winner = match winner {
                Winner::SoleWinner(PlayerHand {
                    name: w_name,
                    hand: w_hand,
                    cards: w_cards,
                }) => {
                    // Compare the current winner (w_...) against the challenger (challenger_...)
                    compare::compare_hands(
                        PlayerHand {
                            name: challenger_name,
                            hand: challenger_hand,
                            cards: challenger_cards,
                        },
                        PlayerHand {
                            name: w_name,
                            hand: w_hand,
                            cards: w_cards,
                        },
                    )
                }
                Winner::Draw(mut draw_winners) => {
                    // It's a draw, compare the challenger against the best hand in the draw group.
                    // Assume the first element of a Draw is the benchmark.
                    let PlayerHand {
                        name: w_name_benchmark,
                        hand: w_hand_benchmark,
                        cards: w_cards_benchmark,
                    } = draw_winners.pop().unwrap();

                    let comparison_result = compare::compare_hands(
                        PlayerHand {
                            name: challenger_name.clone(),
                            hand: challenger_hand.clone(),
                            cards: challenger_cards.clone(),
                        },
                        PlayerHand {
                            name: w_name_benchmark.clone(),
                            hand: w_hand_benchmark.clone(),
                            cards: w_cards_benchmark.clone(),
                        },
                    );

                    // Put the benchmark hand back for future comparisons or draw outcome
                    draw_winners.push(PlayerHand {
                        name: w_name_benchmark,
                        hand: w_hand_benchmark.clone(),
                        cards: w_cards_benchmark,
                    });

                    match comparison_result {
                        Winner::SoleWinner(PlayerHand {
                            name: n,
                            hand: h,
                            cards: c,
                        }) => {
                            if n == challenger_name {
                                // Challenger is better than the benchmark (and thus all previous winners)
                                Winner::SoleWinner(PlayerHand {
                                    name: n,
                                    hand: h,
                                    cards: c,
                                })
                            } else {
                                // Challenger is worse than the benchmark, keep the existing draw group
                                Winner::Draw(draw_winners)
                            }
                        }
                        Winner::Draw(_) => {
                            // Challenger ties with the benchmark, add challenger to the draw group.
                            draw_winners.push(PlayerHand {
                                name: challenger_name,
                                hand: challenger_hand,
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
    /// TODO
    /// + refactor this into several smaller methods,
    fn distribute_pots(&mut self) {
        let winner = self.winner.clone();
        let main_pot = self.pot;
        let side_pots = self.side_pots.clone();
        let ccards = self.community_cards.clone();
        // details of not folded players: names, bests hands, full sets of cards and whether they are all in
        let not_folded: Vec<(PlayerHand, bool)> = self
            .players
            .values()
            .filter(|p| !p.folded)
            .map(|p| {
                let (c1, c2) = p.hole.unwrap();
                let mut cards = ccards.clone();
                cards.extend(vec![c1, c2]);
                (
                    PlayerHand {
                        name: p.name.clone(),
                        hand: compare::best_hand(&cards),
                        cards,
                    },
                    p.all_in,
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
                        hand: best_hand,
                        cards,
                    },
                    _all_in,
                )| PlayerHand {
                    name: name.to_owned(),
                    hand: best_hand.to_owned(),
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
                Winner::SoleWinner(PlayerHand { name, .. }) => {
                    let winner_name = name.clone();
                    if not_folded_clone.iter().any(|(ph, _all_in)| ph.name == name) {
                        // winner is not folded
                        // distribute the main pot
                        *winnings.get_mut(&winner_name).unwrap() += main_pot;
                        if not_all_in.iter().any(|ph| ph.name == winner_name) {
                            // winner is not all in, they win the side pots too
                            let side_pots: usize = self.side_pots.iter().map(|sp| sp.pot).sum();
                            *winnings.get_mut(&winner_name).unwrap() += side_pots;
                        } else {
                            // winner is all in, they only win side pots they contributed to
                            // distribute side pots
                            for sp in side_pots {
                                // possible winners
                                let candidates: Vec<PlayerHand> = not_folded_clone
                                    .iter()
                                    .filter(|(ph, _all_in)| sp.players.contains(&ph.name))
                                    .map(|(ph, _all_in)| PlayerHand {
                                        name: ph.name.to_owned(),
                                        hand: ph.hand.to_owned(),
                                        cards: ph.cards.to_owned(),
                                    })
                                    .collect();
                                if candidates.is_empty() {
                                    // everyone in this side pot has folded so the winnings go to the winner of the main pot
                                    *winnings.get_mut(&winner_name).unwrap() += sp.pot;
                                } else {
                                    // players who participated in this side pot are still in the round
                                    let w = Game::determine_winner(candidates);
                                    match w {
                                        // single winner for this side pot
                                        Winner::SoleWinner(PlayerHand { name, .. }) => {
                                            *winnings.get_mut(&name).unwrap() += sp.pot;
                                        }
                                        // multiple winners for this side pot
                                        Winner::Draw(winners) => {
                                            let pot_share = sp.pot / winners.len();
                                            *winnings.get_mut(&name).unwrap() += pot_share;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        panic!("Winner not in not_folded.");
                    }
                }
                Winner::Draw(winners) => {
                    // distribute main pot
                    let main_pot_share = main_pot / winners.len();
                    for PlayerHand {
                        name,
                        hand: _,
                        cards: _,
                    } in winners.clone()
                    {
                        *winnings.get_mut(&name).unwrap() += main_pot_share;
                    }
                    //distribute side pots
                    for sp in side_pots {
                        // possible winners
                        let candidates: Vec<PlayerHand> = not_folded_clone
                            .iter()
                            .filter(|(ph, _all_in)| sp.players.contains(&ph.name))
                            .map(|(ph, _all_in)| PlayerHand {
                                name: ph.name.to_owned(),
                                hand: ph.hand.to_owned(),
                                cards: ph.cards.to_owned(),
                            })
                            .collect();
                        if candidates.is_empty() {
                            // everyone who contributed to this side pot has folded, the winners share the pot
                            for PlayerHand {
                                name,
                                hand: _,
                                cards: _,
                            } in winners.clone()
                            {
                                *winnings.get_mut(&name).unwrap() += sp.pot;
                            }
                        } else {
                            // there are unfolded players who contributed to this side pot
                            let w = Game::determine_winner(candidates);
                            match w {
                                // single winner for this side pot
                                Winner::SoleWinner(PlayerHand { name, .. }) => {
                                    *winnings.get_mut(&name).unwrap() += sp.pot;
                                }
                                // multiple winners for this side pot
                                Winner::Draw(winners) => {
                                    let pot_share = sp.pot / winners.len();
                                    for PlayerHand { name, .. } in winners {
                                        *winnings.get_mut(&name).unwrap() += pot_share;
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
                    self.players.get_mut(&name).unwrap().bank_roll += pot_share;
                }
            }
            self.pot = 0;
            self.side_pots = Vec::new();
        } else {
            dbg!("Distribute pots called with no winner set.");
        }
    }

    /// Reset the Game and Players after a round.
    fn reset_after_round(&mut self) {
        self.pot = 0;
        self.side_pots = Vec::new();
        self.community_cards = Vec::new();
        self.deck = card::new_deck();
        let mut removed_names: Vec<String> = Vec::new();

        // Loop through the players resetting all_in and folded, and collecting
        // the list of ones that need to be removed for lack of money.
        self.players_order.iter().for_each(|name| {
            let p = self
                .players
                .get_mut(name)
                .expect("Player in order list not found in map.");
            if p.bank_roll == 0 {
                removed_names.push(name.clone());
            } else {
                p.all_in = false;
                p.folded = false;
                p.hole = None;
            }
        });

        // remove player names from self.players_order and Player structs from self.player.
        removed_names.iter().for_each(|name| {
            if let Some(index) = self.players_order.iter().position(|value| value == name) {
                self.players_order.remove(index);
            }
            self.players.remove(name);
        });

        // Assign new dealer.
        let dealer_name = self.dealer.clone().unwrap();
        if !self.players_order.is_empty() {
            let dealer_index = self
                .players_order
                .iter()
                .position(|n| n == &dealer_name)
                .unwrap_or_default();
            let players_order = self.players_order.clone();
            self.dealer = Some(players_order[(dealer_index + 1) % players_order.len()].clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::poker::{
        autoactor::AutoActor,
        betting_strategy::BetArgs,
        card::{BestHand, Card, Hand, Rank, Suit},
    };

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
        let _ = game.join(Player::build("player1", AutoActor::new()));
        let _ = game.join(Player::build("player2", AutoActor::new()));
        if let Err(e) = game.join(Player::build("player3", AutoActor::new())) {
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
        let _ = game.join(Player::build("player1", AutoActor::new()));
        let _ = game.join(Player::build("player2", AutoActor::new()));
        // each player should receive 100 x big blind
        game.players.values().for_each(|p| {
            assert!(
                p.bank_roll == 100 * 20,
                "Expected new player to receive {}, was {}",
                100 * 20,
                p.bank_roll
            )
        });
    }

    #[test]
    fn test_deal_hole_cards() {
        let mut game = Game::build(20, 2);
        let _ = game.join(Player::build("player1", AutoActor::new()));
        let _ = game.join(Player::build("player2", AutoActor::new()));
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
        let _ = game.join(Player::build("player1", AutoActor::new()));
        let _ = game.join(Player::build("player2", AutoActor::build(test_strategy)));
        game.order_players();
        game.deal_hole_cards();

        // Test with players that don't place bets.
        // Both players will use default strategy and check.
        // The pot should contain only the blinds.
        game.place_bets();
        assert!(
            game.pot == 40,
            "Expected game.pot to be 40, was {}",
            game.pot
        );
        game.players.iter().for_each(|(_name, p)| {
            assert!(
                p.bank_roll == 1980,
                "Expected p.bank_roll to be 1980, was {}.",
                p.bank_roll
            );
        });
        game.place_bets();
        assert!(
            game.pot == 80,
            "Expected game.pot to be 80, was {}",
            game.pot
        );
        game.players.iter().for_each(|(_name, p)| {
            assert!(
                p.bank_roll == 1960,
                "Expected p.bank_roll to be 1960, was {}.",
                p.bank_roll
            );
        });
    }

    // A betting strategy that will place a bet if the call is zero
    fn test_strategy(args: BetArgs, _hole_cards: (Card, Card), bank_roll: usize) -> Bet {
        if bank_roll == 0 {
            Bet::Fold
        } else if bank_roll <= args.call {
            Bet::AllIn(bank_roll)
        } else if args.call == 0 {
            Bet::Raise(args.min)
        } else {
            Bet::Call
        }
    }

    #[test]
    fn test_place_bets_modest_strategy() {
        let mut game = Game::build(20, 2);
        let _ = game.join(Player::build("player1", AutoActor::new()));
        let _ = game.join(Player::build("player2", AutoActor::build(test_strategy)));
        game.order_players();
        game.deal_hole_cards();

        game.place_bets();
        assert!(
            game.pot == 40,
            "Expected game.pot to be 40, was {}",
            game.pot
        );
        game.players.iter().for_each(|(_name, p)| {
            assert!(
                p.bank_roll == 1980,
                "Expected p.bank_roll to be 1980, was {}.",
                p.bank_roll
            );
        });
    }

    #[test]
    fn test_place_bets_folded_player() {
        let mut game = Game::build(20, 3);
        let _ = game.join(Player::build("player1", AutoActor::new()));
        let _ = game.join(Player::build("player2", AutoActor::new()));
        let _ = game.join(Player::build("player3", AutoActor::build(test_strategy)));
        game.order_players();
        game.deal_hole_cards();

        // Test with a player who bets once and one which is folded.

        let p2 = game.players.get_mut("player2").unwrap();
        p2.folded = true;

        game.place_bets();
        assert!(
            game.pot == 40,
            "Expected game.pot to be 40, was {}",
            game.pot
        );
        game.players.iter().for_each(|(_name, p)| {
            if p.name == "player2" {
                assert!(
                    p.bank_roll == 2000,
                    "Expected p.bank_roll to be 2000, was {}.",
                    p.bank_roll
                );
            } else {
                assert!(
                    p.bank_roll == 1980,
                    "Expected p.bank_roll to be 1980, was {}.",
                    p.bank_roll
                );
            }
        });
    }

    #[test]
    fn test_deal_flop() {
        let mut game = Game::build(20, 2);
        let _ = game.join(Player::build("player1", AutoActor::new()));
        let _ = game.join(Player::build("player2", AutoActor::new()));
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
        let _ = game.join(Player::build("player1", AutoActor::new()));
        let _ = game.join(Player::build("player2", AutoActor::new()));
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
        let _ = game.join(Player::build("player1", AutoActor::new()));
        let _ = game.join(Player::build("player2", AutoActor::new()));
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
        let _ = game.join(Player::build("player1", AutoActor::new()));
        let _ = game.join(Player::build("player2", AutoActor::new()));

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

        let w = &game.winner;

        if let Some(Winner::SoleWinner(PlayerHand {
            name: n,
            hand: h,
            cards: _cs,
        })) = w
        {
            assert!(n == "player1", "Expected player1, was {}", n);
            assert!(
                h.hand
                    == Hand::Flush(
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

        let w = &game.winner;

        if let Some(Winner::Draw(winners)) = w {
            assert!(
                winners.len() == 2,
                "Expected 2 winners, was {}",
                winners.len()
            );
            winners.iter().for_each(
                |PlayerHand {
                     name: _name,
                     hand: h,
                     cards: _cs,
                 }| {
                    assert!(
                        h.hand == Hand::OnePair(Rank::Rank10),
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
        let _ = game.join(Player::build("player1", AutoActor::new()));
        let _ = game.join(Player::build("player2", AutoActor::new()));

        game.deal_hole_cards();
        // test outight winner
        game.pot = 120;
        game.winner = Some(Winner::SoleWinner(PlayerHand {
            name: "player1".to_string(),
            hand: BestHand {
                hand: Hand::HighCard(Rank::Ace),
                cards: Vec::new(),
            },
            cards: Vec::new(),
        }));

        game.distribute_pots();

        assert!(game.pot == 0, "Expected game.pot == 0, was {}", game.pot);
        assert!(
            game.side_pots.is_empty(),
            "Expected no side pots, was {:?}",
            game.side_pots
        );

        let w = game.winner.clone();

        if let Some(Winner::SoleWinner(PlayerHand {
            name,
            hand: _hand,
            cards: _cards,
        })) = w
        {
            let p = game.players.get(&name).unwrap();
            assert!(
                p.bank_roll == 2120,
                "Expected winner bankroll to be 2120, was {}",
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
            PlayerHand {
                name: "player1".to_string(),
                hand: BestHand {
                    hand: Hand::HighCard(Rank::Ace),
                    cards: Vec::new(),
                },
                cards: Vec::new(),
            },
            PlayerHand {
                name: "player2".to_string(),
                hand: BestHand {
                    hand: Hand::HighCard(Rank::Ace),
                    cards: Vec::new(),
                },
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
                     hand: _h,
                     cards: _cs,
                 }| {
                    let p = game.players.get(name).unwrap();
                    assert!(
                        p.bank_roll == 60,
                        "Expected player to have bankroll == 60, was {}.",
                        p.bank_roll
                    );
                },
            );
        } else {
            panic!("Expected a draw.");
        }

        // test a draw with a side pot

        let _ = game.join(Player::build("player3", AutoActor::new()));
        //game.deal_hole_cards();
        // players 2 and 3 are all in
        game.players.iter_mut().for_each(|(_name, p)| {
            p.bank_roll = 0;
            p.hole = Some((
                Card {
                    rank: Rank::Rank2,
                    suit: Suit::Clubs,
                },
                Card {
                    rank: Rank::Rank3,
                    suit: Suit::Clubs,
                },
            ));
            if p.name == "player2" || p.name == "player3" {
                p.all_in = true;
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
                hand: BestHand {
                    hand: Hand::HighCard(Rank::Ace),
                    cards: Vec::new(),
                },
                cards: Vec::new(),
            },
            PlayerHand {
                name: "player2".to_string(),
                hand: BestHand {
                    hand: Hand::HighCard(Rank::Ace),
                    cards: Vec::new(),
                },
                cards: Vec::new(),
            },
            PlayerHand {
                name: "player3".to_string(),
                hand: BestHand {
                    hand: Hand::HighCard(Rank::Ace),
                    cards: Vec::new(),
                },
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
                     hand: _h,
                     cards: _cs,
                 }| {
                    let p = game.players.get(name).unwrap();
                    if p.name == "player1" || p.name == "player3" {
                        assert!(
                            p.bank_roll == 70,
                            "Expected non-all inplayer to split main pot (120/3) and side pot (60/2) = 70, was {}.",
                            p.bank_roll
                        );
                    } else {
                        assert!(
                            p.bank_roll == 40,
                            "Expected all in player to split main pot (120/3) = 40, was {}.",
                            p.bank_roll
                        );
                    }
                },
            );
        } else {
            panic!("Expected a draw.");
        }
    }

    #[test]
    fn test_reset_after_round() {
        let mut game = Game::build(20, 4);
        let _ = game.join(Player::build("player1", AutoActor::new()));
        let _ = game.join(Player::build("player2", AutoActor::new()));
        let _ = game.join(Player::build("player3", AutoActor::new()));
        let _ = game.join(Player::build("player4", AutoActor::new()));
        game.play_round();
        let dealer_first = game.dealer.clone().unwrap();
        game.players.get_mut("player3").unwrap().bank_roll = 0;
        game.reset_after_round();
        assert!(
            !game.players.contains_key("player3"),
            "Expected player3 to be removed"
        );
        let found_player3 = game.players_order.contains(&"player3".to_string());
        assert!(
            !found_player3,
            "Expected player3 to be removed, was {:?}",
            game.players_order
        );
        assert!(
            game.pot == 0,
            "Expected game.pot to be zero, was {}",
            game.pot
        );
        assert!(
            game.side_pots.is_empty(),
            "Expected game.side_pots to be empty, was {:?}",
            game.side_pots
        );
        assert!(
            game.community_cards.is_empty(),
            "Expected game.community_cards to be empty, was {:?}",
            game.community_cards
        );
        assert!(
            game.deck.len() == 52,
            "Expected game.deck to have 52 cards, was {}",
            game.deck.len()
        );
        let dealer_second = game.dealer.clone().unwrap();
        assert!(
            dealer_first != dealer_second,
            "Expected dealer to have changed from {}",
            dealer_first
        );
        game.players.values().for_each(|p| {
            assert!(!p.folded, "Player should not be folded: {:?}", p);
            assert!(!p.all_in, "Player should not be all_in: {:?}", p);
        });
    }
}
