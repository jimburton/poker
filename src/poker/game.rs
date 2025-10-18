use crate::poker::card::{new_deck, Card, Hand};
use crate::poker::compare::{best_hand, compare_hands};
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
    players: HashMap<String, Player>,
    dealer: Option<String>,
    current_player: Option<Player>,
    buy_in: usize,
    call: usize,
    pot: usize,
    side_pot: usize,
    deck: Vec<Card>,
    community_cards: Vec<Card>,
    num_players: u8,
    side_pot_active: bool,
    winner: Option<Winner>,
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
            deck: Vec::new(),
            community_cards: Vec::new(),
            num_players,
            side_pot_active: false,
            winner: None,
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
        self.distribute_pot();
    }

    fn players_buy_in(&mut self) {
        self.players.iter_mut().for_each(|(_, p)| {
            if p.bank_roll > self.buy_in {
                p.bank_roll -= self.buy_in;
                self.pot += self.buy_in;
            } else if p.bank_roll > 0 {
                self.pot += p.bank_roll;
                p.bank_roll = 0;
                p.all_in = true;
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
                self.call = self.buy_in;
            }
            // players always bet if they are first to go, then always call
            if p.bank_roll > self.call {
                p.bank_roll -= self.call;
                if self.side_pot_active && !p.all_in {
                    self.side_pot += self.call;
                } else {
                    self.pot += self.call;
                }
            } else if p.bank_roll > 0 {
                p.all_in = true;
                self.side_pot_active = true;
                self.pot += p.bank_roll;
                p.bank_roll = 0;
            }
        });
    }

    /// Determines the winner(s) of the hand.
    pub fn showdown(&mut self) {
        // Calculate the best hand for each non-folded player.
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

        // Initialize the winner with the first player's hand, consuming it from the vector.
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
                        // Challenger data is moved here
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
                        // FIX: Clone challenger data for comparison call, so the originals are still
                        // available to be moved into draw_winners if the result is a draw.
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

        dbg!("Final Showdown Result: {:?}", &winner);
        self.winner = Some(winner);
    }

    /// Distributes the pot and side pot to the winner(s) and resets the pot amounts.
    pub fn distribute_pot(&mut self) {
        if let Some(winner) = &self.winner {
            match winner {
                Winner::Winner { name, .. } => {
                    let total_pot = self.pot + self.side_pot;
                    if let Some(player) = self.players.get_mut(name) {
                        player.bank_roll += total_pot;
                        println!("\n--- Pot Distribution (outright winner) ---");
                        println!("{} wins {} chips!", name, total_pot);
                    } else {
                        println!("Error: Winner {} not found in player list.", name);
                    }
                }
                Winner::Draw(draw_winners) => {
                    let total_pot = self.pot + self.side_pot;
                    let num_winners = draw_winners.len();
                    let num_winners_not_all_in = draw_winners
                        .iter()
                        .filter(|(name, hand, cards)| {
                            println!("Looking for player: {}", name);
                            let p = self.players.get(name).expect("Couldn't find player");
                            !p.all_in
                        })
                        .collect::<Vec<_>>()
                        .len();

                    if num_winners == 0 {
                        println!("Error: Draw with no winners found.");
                        return;
                    }

                    let main_pot_share = self.pot / num_winners;
                    let side_pot_share = self.side_pot / num_winners_not_all_in;
                    let remainder = total_pot % num_winners;

                    println!("\n--- Pot Distribution (draw) ---");
                    println!(
                        "Draw between {} players. Each player not all-in receives {} chips.",
                        num_winners,
                        main_pot_share + side_pot_share
                    );
                    println!("Each all-in player receives {} chips.", main_pot_share);

                    // Distribute the share
                    for (name, _, _) in draw_winners.into_iter() {
                        if let Some(player) = self.players.get_mut(name) {
                            if player.all_in {
                                player.bank_roll += main_pot_share;
                            } else {
                                player.bank_roll += main_pot_share + side_pot_share;
                            }
                        }
                    }

                    // Simplified remainder handling: just log it and drop it for now
                    if remainder > 0 {
                        println!(
                            "Note: {} chips remainder in the pot (unhandled in this implementation).",
                            remainder
                        );
                    }
                }
            }

            // Reset pots
            self.pot = 0;
            self.side_pot = 0;
            println!("Pots reset to 0.");
        } else {
            dbg!("Winner not chosen.");
        }
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
        game.players.iter().for_each(|(name, p)| {
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
            name: name,
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

        game.players.iter_mut().for_each(|(name, p)| {
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
        game.players.iter_mut().for_each(|(name, p)| {
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
