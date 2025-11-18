# Poker library (Texas Hold 'Em)

The key structs are [`Player`](./src/poker/player.rs) and
[`Game`](./src/poker/game.rs). There are a couple of utility functions
in [src/poker/mod.rs](./src/poker/mod.rs) that enable you to create a
new `Game` instance populated with players. You need to supply one or
more `Player` instances. For example, using `CLIActor` from the
[`poker_cli`](../poker_cli/) crate to play a game in the terminal:

```rust
use poker::poker::{
    game::Game,
    new_game_one_player,
    player::Player
};
use cli::player::CLIActor;

fn main() {
  let actor = CLIActor::new();
  let p = Player::build("Me", actor);
  let mut g = poker::new_game_one_player(p, 100, 3);
  g.play();
}
```

`Game` requests bets from `Player` instances by calling
`Player::place_bet`. `Player` then passes off that task to the object
in its `actor` field, which is an object implementing the `Actor`
trait. In this way, `Game` doesn't know or care whether a player is
placing bets according to some algorithm or is an interactive player
located in a client. The library includes
[`AutoActor`](./src/poker/autoactor.rs), which can be configured with
various [betting strategies](./src/poker/betting_strategy.rs). Actors
written to be used interactively in clients include
[`CLIActor`](../poker_cli/src/cli/actor.rs) in the `poker_cli` crate
(which asks the user to enter a bet in the terminal) and
[`RemoteActor`](../poker_server/src/server/actor.rs) in the
`poker_server` crate (which sends the bet request over a websocket
connection and waits for the response).

## Poker terminology

Disclaimer: as I don't play Texas Hold 'Em (or any other form of
poker) I've needed to work this out online, and one or two of the
details seem to vary across sources. I've taken the simpler option
where possible and the following is just the way this library works.
  
When a game begins the cost of participating in a round, or *big
blind*, is fixed. This also establishes the *buy in* and the
*minimum bet*. The buy in is the number of chips a player receives
when they join the game, and is set at 100 times the big blind. The
minimum bet is the same as the big blind.
  
Players that run out of chips are removed at the end of each round.
  
### Rounds

  A game consists of any number of *rounds* and ends when there is only
  one player left. 
  
  At the beginning of the first round the *dealer button* is assigned
  to one of the players. In each successive round the dealer button is
  passed to the left.
  
  Before the round begins, each player needs to *ante up* by paying
  into the pot. The person to the left of the dealer pays the *small
  blind*, which is set at half of the big blind. If any player has too
  few chips to pay the appropriate amount, they pay all of their chips
  and are *all in* for this round (see below).
   
  The *stages* of each round are as follows:
  
  + `PreFlop`: each player is dealt two cards "face down" (which only
    they can see) and a round of betting takes place.
  + `Flop`: three community cards (i.e. cards which are visible to all
    players) are dealt. A second round of betting takes place.
  + `Turn`: a fourth community card is dealt. A third round of betting
    takes place.
  + `River`: a fifth community card is dealt. A final round of betting
    takes place.
  + `Showdown`: the best hand of the players who have not folded wins
    the round. Winnings are distributed. Players with no chips leave
    the game.

### Hands and winning the round.

A player's best hand is the best one that can be made using their two
hole cards and the five community cards. The hands are ranked in the
standard way (see
[https://en.wikipedia.org/wiki/List_of_poker_hands](https://en.wikipedia.org/wiki/List_of_poker_hands)). There
are no jokers in the game, so the best hand is a Royal Flush.

### Betting

  The player to the left of the dealer acts first when placing
  bets. On a player's turn to bet, if no bet has yet been made in this
  round of betting they can:
  
  + check -- pass the action without contributing to the pot,
  + fold -- retire from the round,
  + bet -- make a bet equal to the minimum bet or more,
  + go all in -- pay all of their chips into the pot. This amount may
    or may not be less than the minimum bet. Players who have gone all
    in are eligible to win the round but don't participate in betting
    from that point on.
  
  If a bet has been made in this round then the player whose turn it
  is to bet must take one of the following actions:
  
  + fold,
  + call -- match the amount of the current bet,
  + raise -- pay an amount greater than the current bet,
  + go all in.
  
  When a player raises the bet, everyone else has to respond (by
  calling, folding, raising or going all in), so a round of betting
  can theoretically continue until everyone is all in.
  
  Each time a player goes all in, a *side pot* is created and
  subsequent bets are paid into it. In normal circumstances, players
  may only receive a share of pots to which they contributed. E.g. if
  the first player to go all in is among the winners, they receive a
  share of the main pot but not of any side pots created after they
  went all in. 
  
  If, at the end of the round, a side pot exists to which the
  winner(s) did not contribute, the money in that side pot is
  distributed to the player(s) with the best hands amongst those who
  contributed to it and are still in the game.

# Notes on improving the code

+ Implement `Copy` trait for types that contain only promitive values or
  other `Copy` types,
+ Use slices rather than vectors where possible (e.g. `&[Card]` not
  `Vec<Card>`).
+ Use `Rc<T>` for cheap sharing. Not sure where I can apply this.
  
# TODO

+ Refactor long functions, especially `Game::distribute_pot`.
+ Add a new kind of game that runs for a fixed number of rounds and
  bumps up the blind periodically. The winner is the player with the
  highest bank roll. E.g. a game that runs for 50 rounds, with the
  blind doubling every 5 rounds. New auto players are added after each
  round as necessary to keep the numbers up.
