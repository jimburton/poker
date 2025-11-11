# Websocket server for the poker library.

See the [docs](../docs/poker_server/index.html).

Run the server with:

```
$ cargo run --bin poker_server
```

The server accepts several options:

```
$ cargo run --bin poker_server -- --help
Usage: target/debug/poker_server [options]

Options:
    -c, --config PATH   set the config file location
    -l, --log PATH      set the log file location
    -n, --host NAME     set the host name
    -p, --port NUMBER   set the port number
    -h, --help          print this help menu
```

The server can also be configured by making changes to the files
[poker.toml](conf/poker.toml) (which configures the hostname and port)
and [logging_conf.yaml](conf/logging_conf.yaml), then using the
command line flags to tell the server where the files are, or putting
them in the default location `~/conf/poker/`.

Initial axum/ws code from
[https://github.com/0Itsuki0/Rust_AxumWebSocket](https://github.com/0Itsuki0/Rust_AxumWebSocket).

## Client and Server messages

After connecting, clients need to send a `NewGame` message:

```json
{"NewGame":{"name":"James"}}

```

The server responds with a `PlayerMessage` confirming their name and bank roll. E.g.
  `{"Player":{"name":"James","bank_roll":10000}}`.
  
From then on, the following messages are sent in each round.

At the beginning of each round, two messages are sent:

+ A `PlayersInfo` message, listing all players and their bank rolls, and the name of the
  dealer. E.g. `{"PlayersInfo":{"players":[["Bob",9950],["Cali",9900],["Alice",9900],["James",9900]],"dealer":"James"}}`.
+ A `HoleCards` message describing the player's hole
  cards. E.g. `{"HoleCards":{"cards":[{"rank":"Rank6","suit":"Clubs"},{"rank":"Rank4","suit":"Diamonds"}]}}`.
  
Within each round there are several stages. These messages are sent in
each of them:

+ A `StageDecl` message declaring the current stage (one of `PreFlop`,
  `Flop`, ``Turn`, `River` and `ShowDown`) and the current set of
  community cards. E.g. `{"StageDecl":{"stage":"PreFlop","community_cards":[]}}`.
+ Several `BetPlaced` messages indicating that a player placed a
  bet. E.g. `{"BetPlaced":{"player":"Cali","bet":"Call"}}`.
+ A `PlaceBet` message requesting that the client places a bet. This
  has the following information:

  ```
  {"PlaceBet":
     {"args":
        {"call":0, # the current requirement to continue without folding
	     "min":100, # the minimum bet
	     "stage":"PreFlop",
	     "cycle":0, # the number of times each player has been asked
     to bet in this round
	     "community_cards":[]
     },
	  "hole_cards":[{"rank":"Rank6","suit":"Clubs"},
	                {"rank":"Rank4","suit":"Diamonds"}],
	  "bank_roll":9900,
	  "best_hand":{"HighCard":"Rank6"}
    }
  }
  ```
+ At this stage the client needs to respond with a `PlayerBet`
  message, which should be one of:
  
  + `{"PlayerBet":"Fold"}`,
  + `{"PlayerBet":"Check"}`,
  + `{"PlayerBet":{"Call":number}}`,
  + `{"PlayerBet":{"Raise":number}}`,
  + `{"PlayerBet":{"AllIn":number}}`.
  
  Depending on how often people raise the bet, the client may be asked to
  place several bets in each stage. The next stage begins with a new
  `StageDecl` message.
  
  Eventually the round is decided (either because all but one players
  folded or because we reached the `ShowDown` and hands were compared)
  and a `RoundWinner` message is sent. E.g.
  
  ```
  {"RoundWinner":
    {"winner":
	  {"SoleWinner":
	    {"name":"James",
		 "hand":
		   {"TwoPair":["Jack","Rank4"]},
		 "cards":[{"rank":"Rank4","suit":"Hearts"},
			      {"rank":"Jack","suit":"Clubs"},
				  {"rank":"Rank9","suit":"Clubs"},
				  {"rank":"Jack","suit":"Hearts"},
				  {"rank":"Queen","suit":"Diamonds"},
				  {"rank":"Rank6","suit":"Clubs"},
				  {"rank":"Rank4","suit":"Diamonds"}]
		}
	  }
    }
  }

  ```
  
  When there is only one player left, a `GameWinner` message is ent,
  having the same format as `RoundWinner` and the server closes the
  connection.
  
```
$ npm install wscat
$ wscat -c ws://localhost:3000
Connected (press CTRL+C to quit)
> {"NewGame":{"name":"James"}}
< {"PlayerUpdate":{"player":"James","bet":{"Raise":200}}}
< {"General":{"msg":{"PlayersInfo":[["Player 2",9900],["Player 1",9950],["James",9900],["Player 3",9900]]}}}
< {"PlaceBet":{"args":{"call":0,"min":100,"stage":"PreFlop","cycle":0,"community_cards":[]},"hole_cards":[{"rank":"Jack","suit":"Hearts"},{"rank":"Rank2","suit":"Hearts"}],"bank_roll":9900,"best_hand":{"HighCard":"Jack"}}}
> {"PlayerBet":"Check"}
> {"PlayerBet":{"Raise":200}}
```
