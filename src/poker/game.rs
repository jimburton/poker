use rand::rng;
use rand::seq::SliceRandom;
use std::collections::HashMap;

use crate::poker::types::{Card, Game, Player, Rank, Suit};

pub fn play() -> Game {
    let mut game = Game::new(10, 5);
    let mut deck = new_deck();
    let mut rng = rng();
    deck.shuffle(&mut rng);
    println!("Deck: {:?}", deck);

    let mut players: HashMap<String, Player> = HashMap::new();

    players.insert("James".to_string(), Player::new("James", 100));
    players.insert("Bob".to_string(), Player::new("Bob", 100));
    players.insert("Alice".to_string(), Player::new("Alice", 100));
    players.insert("Dileas".to_string(), Player::new("Dileas", 100));
    players.insert("Terry".to_string(), Player::new("Terry", 100));
    game.players = players;

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

pub fn deal_hole_cards(deck: &mut Vec<Card>, players: &mut HashMap<String, Player>) -> () {
    players.iter_mut().for_each(|(_, p)| {
        let c1 = deck.pop().unwrap();
        let c2 = deck.pop().unwrap();
        p.hole = Some((c1, c2));
    });
}
