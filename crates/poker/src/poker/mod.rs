use autoactor::AutoActor;
use player::Player;

pub mod autoactor;
pub mod betting_strategy;
pub mod card;
pub mod compare;
pub mod game;
pub mod names;
pub mod player;
pub mod sequence;
mod test_data;

/// Create a new game with one supplied player and the supplied number of auto players.
/// Supply an interactive player to create a one player game.
///
/// TODO: uniquify names.
pub fn new_game_one_player(player: Player, big_blind: usize, num_auto_players: u8) -> game::Game {
    let mut g = game::Game::build(big_blind, num_auto_players + 1);
    g.join(player).unwrap_or_else(|e| eprintln!("{e:?}"));
    let prefix = "Player ".to_string();
    for i in 1..=num_auto_players {
        let mut name = prefix.clone();
        name.push_str(&i.to_string());
        let auto_player = Player::build(&name, AutoActor::new());
        g.join(auto_player).unwrap_or_else(|e| eprintln!("{e:?}"));
    }
    g
}

/// Create a new game with the supplied players
///
/// TODO: uniquify names.
pub fn new_game_with_players(players: Vec<Player>, big_blind: usize) -> game::Game {
    let mut g = game::Game::build(big_blind, players.len() as u8);
    for p in players {
        g.join(p).unwrap_or_else(|e| eprintln!("{e:?}"));
    }
    g
}

/// Utility function that rotates a vector (V) by a given index (I).
/// The rotation moves the elements starting from V[I] to the front,
/// followed by the elements V[..I].
///
/// # Type Parameters
/// * `T`: The element type, which must implement `Clone` to allow copying
///   elements into the new output vector.
///
/// # Arguments
/// * `v`: A slice reference to the input vector.
/// * `i`: The index to start the rotation from.
///
/// # Returns
/// A new `Vec<T>` containing the rotated elements.
pub fn rotate_vector<T: Clone>(v: &[T], i: usize) -> Vec<T> {
    // Handle the empty vector case immediately
    if v.is_empty() {
        return Vec::new();
    }

    // Ensure the index is within the vector's bounds by using the modulo operator.
    // This allows large 'i' values to wrap around (circular rotation).
    let index = i % v.len();

    // Use split_at to divide the slice into two parts at the rotation point.
    // tail: v[index..] (The part that moves to the front)
    // head: v[..index] (The part that moves to the back)
    let (head, tail) = v.split_at(index);

    // Start the new vector with the 'tail' part. We use to_vec() here
    // to create the new vector and copy the elements.
    let mut rotated = tail.to_vec();

    // Append the 'head' part to the end of the new vector.
    rotated.extend_from_slice(head);

    rotated
}
