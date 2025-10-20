use std::collections::HashMap;

use poker::poker::betting_strategy::{modest_betting_strategy, six_max};
use poker::poker::game::Game;

fn main() {
    let names: Vec<String> = vec![
        "James".to_string(),
        "Bob".to_string(),
        "Alice".to_string(),
        "Dileas".to_string(),
        "Evie".to_string(),
    ];
    let mut results: HashMap<String, (usize, usize)> = HashMap::new();
    for name in names {
        results.insert(name, (0, 0));
    }
    for _ in (0..5).collect::<std::vec::Vec<i32>>() {
        let mut g: Game = Game::build(100, 5);
        g.join("James").unwrap_or_else(|e| eprintln!("{e:?}"));
        g.join("Bob").unwrap_or_else(|e| eprintln!("{e:?}"));
        g.join("Alice").unwrap_or_else(|e| eprintln!("{e:?}"));
        g.join_with_strategy("Dileas", modest_betting_strategy)
            .unwrap_or_else(|e| eprintln!("{e:?}"));
        g.join_with_strategy("Evie", six_max)
            .unwrap_or_else(|e| eprintln!("{e:?}"));

        let winner = g.play();
        println!("{:?}", winner);
        if let Some((won, winnings)) = results.get(&winner.name) {
            results.insert(winner.name, (won + 1, winnings + winner.winnings));
        } else {
            println!("Couldn't update results.");
        }
    }
    for (name, (won, winnings)) in results {
        println!("{} won {} games, total winnings {}", name, won, winnings);
    }
}
