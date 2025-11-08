Initial axum/ws code from
https://github.com/0Itsuki0/Rust_AxumWebSocket

$ npm install wscat
$ wscat -c ws://localhost:3000
Connected (press CTRL+C to quit)
> Hello
< Echo back text: Hello

Form of the serialised structs:

> {"NewGame":{"name":"James"}}
< {"PlayerUpdate":{"player":"James","bet":{"Raise":200}}}
< {"General":{"msg":{"PlayersInfo":[["Player 2",9900],["Player 1",9950],["James",9900],["Player 3",9900]]}}}
< {"PlaceBet":{"args":{"call":0,"min":100,"stage":"PreFlop","cycle":0,"community_cards":[]},"hole_cards":[{"rank":"Jack","suit":"Hearts"},{"rank":"Rank2","suit":"Hearts"}],"bank_roll":9900,"best_hand":{"HighCard":"Jack"}}}
> {"PlayerBet":"Check"}
> {"PlayerBet":{"Raise":200}}

web client

npm ci
npm run dev
