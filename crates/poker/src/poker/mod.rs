use autoactor::AutoActor;
use game::Game;
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
/// The auto players use either the `six_max` or `modest` betting strategies.
pub fn new_game_one_player(player: Player, big_blind: usize, num_auto_players: u8) -> Game {
    let mut g = Game::build(big_blind, num_auto_players + 1);
    g.join(player).unwrap_or_else(|e| eprintln!("{e:?}"));
    // make an iterator of actors using different strategies.
    let actors = (0..num_auto_players).map(|i| {
        if i % 2 == 0 {
            AutoActor::build(betting_strategy::six_max)
        } else {
            AutoActor::build(betting_strategy::modest_betting_strategy)
        }
    });
    let names = names::get_names(num_auto_players as usize).unwrap();
    // zip the names and the actors.
    let names_actors = names.iter().zip(actors);
    names_actors.for_each(|(name, actor)| {
        let auto_player = Player::build(name, actor);
        g.join(auto_player).unwrap_or_else(|e| eprintln!("{e:?}"));
    });
    g
}

/// Create a new game with the supplied players
pub fn new_game_with_players(players: Vec<Player>, big_blind: usize) -> Game {
    let mut g = Game::build(big_blind, players.len() as u8);
    for p in players {
        g.join(p).unwrap_or_else(|e| eprintln!("{e:?}"));
    }
    g
}

/// Rotate a vector (V) by a given index (I).
/// The rotation moves the elements starting from V[I] to the front,
/// followed by the elements V[..I].
///
/// # Returns
/// A new `Vec<T>` containing the rotated elements.
pub fn rotate_vector<T: Clone>(v: &[T], i: usize) -> Vec<T> {
    if v.is_empty() {
        return Vec::new();
    }
    let index = i % v.len();
    let (head, tail) = v.split_at(index);
    let mut rotated = tail.to_vec();
    rotated.extend_from_slice(head);
    rotated
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_vector() {
        let vec1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let i = 4;
        let result = rotate_vector(&vec1, i);
        assert!(
            result.len() == vec1.len(),
            "Expected result to be the same length as input ({}), was {}",
            vec1.len(),
            result.len()
        );
        (0..(vec1.len())).for_each(|j| {
            assert!(
                vec1[j] == result[(j + i + 1) % vec1.len()],
                "Expected elements in same order, was {:?} vs {:?}",
                vec1,
                result,
            )
        });
    }
}
