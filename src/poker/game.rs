use rand::rng;
use rand::seq::SliceRandom;
use std::collections::HashMap;

use crate::poker::types::{Card, Game, Player, Rank, Suit};

pub fn new_game(buy_in: usize, num_players: u8) -> Game {
    Game::build(buy_in, num_players)
}

pub fn join<'a>(
    name: &str,
    bank_roll: usize,
    game: &'a mut Game,
) -> Result<&'a mut Game, &'static str> {
    game.add_player(Player::build(name, bank_roll))
        .or_else(|err| return Err(err));
    Ok(game)
}

pub fn play(mut game: Game) -> Game {
    let mut deck = new_deck();
    let mut rng = rng();
    deck.shuffle(&mut rng);
    println!("Deck: {:?}", deck);

    deal_hole_cards(&mut deck, &mut game.players);

    println!("Game: {:?}", game);

    game
}

fn new_deck() -> Vec<Card> {
    Rank::values()
        .iter()
        .flat_map(|i| Suit::values().map(move |j| Card { rank: *i, suit: j }))
        .collect()
}

pub fn deal_hole_cards(deck: &mut Vec<Card>, players: &mut HashMap<String, Player>) {
    players.iter_mut().for_each(|(_, p)| {
        let c1 = deck.pop().unwrap();
        let c2 = deck.pop().unwrap();
        p.hole = Some((c1, c2));
    });
}
