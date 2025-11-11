# Poker library (Texas Hold 'Em)

See the [docs](../docs/poker/index.html)

## Joining and leaving the game

  Before a game begins the cost of participating in a round, or *big
  blind*, is fixed. This also establishes the *buy in* and the
  *minimum bet*. The buy in is the number of chips a player receives
  when they join the game, and is set at 100 times the big blind.
  
  Players that run out of chips are removed at the end of each round.
  
## Rounds

  A game consists of any number of rounds and ends when there is only
  one player left. 
  
  At the beginning of the first round the *dealer button* is assigned
  to one of the players. In each successive round the dealer button is
  passed to the left.
  
  Before the round begins, each player needs to *ante up* by paying
  into the pot. The person to the left of the dealer pays the *small
  blind*, which is set at half of the big blind. If any player has too
  few chips to pay the appropriate amount, they pay all of their chips
  and are *all in* for this round (see below).
   
  The stages of each round are as follows:
  
  + Hole cards: Each player is dealt two cards "face down".
  + PreFlop: a round of betting takes place.
  + Flop: three community cards (visible to all) are dealt. A second
    round of betting takes place.
  + Turn: a fourth community card is dealt. A third round of betting
    takes place.
  + River: a fifth community card is dealt. A final round of betting
    takes place.
  + Showdown: the best hand of the players who have not folded wins
    the round.
  + Winnings are distributed. Players with no chips leave the game.

## Hands and winning the round.

A player's best hand is the best one that can be made using their two
hole cards and the five community cards. The hands are ranked in the
standard way, see
[https://en.wikipedia.org/wiki/List_of_poker_hands](https://en.wikipedia.org/wiki/List_of_poker_hands). There
are no jokers in the game, so the best hand is a Royal Flush.

## Betting

  In this implementation, the player to the left of the dealer acts
  first when placing bets.
  
  On a player's turn to bet, if no bet has yet been made in this round
  of betting they can:
  
  + check -- pass the action without contributing to the pot,
  + fold -- retire from this round,
  + bet -- make a bet equal to the minimum bet or more,
  + go all in -- pay all of their chips into the pot. This amount may
    or may not be less than the minimum bet.
  
  If a bet has been made in this round then they must take one of the
  following actions:
  
  + fold,
  + call -- pay the amount of the current bet,
  + raise -- pay an amount greater than the current bet,
  + go all in.
  
  When a player raises the bet everyone else has to call, fold or raise,
  so this can theoretically go round in a circle until everyone is all in.
  
  When a player goes all in by betting their entire bank roll, a side
  pot becomes active and subsequent bets are paid into it. Each time a
  player goes all in, a new side pot is created. Winning players may
  only receive a share of a pot which they contributed to. I.e. if the
  first player to go all in is among the winners, they may only
  receive a share of the main pot. 
  
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
