# Texas Hold 'Em in Rust

An exercise in learning Rust. The code is organised as follows:

+ the game logic in a library,
+ a server that uses the library and provides an API for clients,
+ a CLI client for testing purposes,
+ a web client written using React.

I don't play poker, so the way I've implemented it might not be
standard in various ways.

## Sequence of events in a game

+ **Joining and leaving the game**

  Before a game begins the cost of participating in a round, or *big
  blind*, is fixed. This also establishes the *buy in* and the
  *minimum bet*. The buy in is the number of chips a player receives
  when they join the game, and is set at 100 times the big blind.
  
  New players may join at the beginning of a round. Players that run
  out of chips are removed.
  
+ **Rounds**

  A game consists of any number of rounds. At the beginning of the
  first round the *dealer button* is assigned to one of the
  players. In each successive round the dealer button is passed to the
  left.
  
  Before the round begins, each player needs to *ante up* by paying
  into the pot. The person to the left of the dealer pays the *small
  blind*, which is set at half of the big blind. Confusingly, this
  player is known as the small blind. If any player has too few chips
  to pay the appropriate amount, they pay all of their chips and are
  *all in* for this round (see below). 
   
  The stages of each round are as follows:
  
  + Hole cards: Each player is dealt two cards "face down".
  + PreFlop: a round of betting takes place.
  + Flop: three community cards (visible to all) are dealt. A round of
    betting takes place.
  + Turn: a fourth community card is dealt. A round of betting takes
    place.
  + River: a fifth community card is dealt. A final round of betting
    takes place.
  + Showdown: the best hand of the players who have not folded wins
    the round.
  + Winnings are distributed.
	
+ **Betting**

  In this implementation, the small blind acts first when placing
  bets. (Some explanations state that the player to the left of the
  big blind should act first, where the big blind is the player to the
  left of the small blind...)
  
  On a players turn to bet, then if no bet has been made they can:
  + check -- pass the action without contributing to the pot,
  + fold -- retire from this round,
  + bet -- make a bet equal to the big blind or more,
  + go all in -- pay all of their chips into the pot. This amount may
    or may not be less than the minimum bet.
  
  If a bet has been made in this round then they must
  + fold,
  + call -- pay the amount of the current bet,
  + raise -- pay an amount greater than the current bet,
  + go all in.
  
  When a player raises the bet everyone else has to call, fold or raise,
  so this can go round in a circle until everyone is all in.
  
  When a player goes all in by betting their entire bank roll, a side
  pot becomes active and subsequent bets are paid into it. Each time a
  player goes all in, a new side pot is created.
  
  If an all-in player wins the round, they win the main pot plus any side pots
  they contributed to (or a share thereof if there is a draw). If side pots
  exist which the winner did not contribute to, the money in that side pot
  is distributed to the player(s) with the best hands amongst those who
  contributed to it and are still in the game.
  
  If, at the end of a round, a side pot exists where none of the players
  who contributed to it are still in the game, that side pot is distributed
  among the winners of the main pot.
  
# TODO

+ Add a new kind of game that runs for a fixed number of rounds and
  bumps up the blind periodically. The winner is the player with the
  highest bank roll. E.g. a game that runs for 50 rounds, with the
  blind doubling every 5 rounds. New auto players are added after each
  round as necessary to keep the numbers up.
