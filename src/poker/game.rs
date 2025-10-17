use rand::seq::SliceRandom;
use std::collections::HashMap;

use crate::poker::types::{Card, Game, Player, Rank, Suit};

pub fn new_game(buy_in: usize, num_players: u8) -> Game {
    Game::build(buy_in, num_players)
}
