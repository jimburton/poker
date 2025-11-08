/*
Messages the client can send relate to placing bets:

{"PlayerBet":<Bet>}
Bet: "Check"
     | "Fold"
     | "Call"
     | {"Raise":<amount>}
     | {"AllIn":<amount>}

Messages the client receives from the server:

{"Error":<string>}
{"PlaceBet":
	{"args":
		{"call":<amount>,
		"min":<amount>,
		"stage":<Stage>,
		"cycle":<amount>,
		"community_cards":[<Card> .. zero to five]},
	"hole_cards":[<Card> .. two],
	"bank_roll":<amount>,
	"best_hand":Hand}}
Rank: "Rank2" .. "Rank10"
      | "Jack"
      | "Queen"
      | "King"
      | "Ace"
Stage: "PreFlop"
       | "Flop"
       | "Turn"
       | "River"
       | "ShowDown"
Suit: "Clubs"
      | "Spades"
      | "Diamonds"
      | "Hearts"
Card: {"rank":<Rank>,"suit":<Suit>}
Hand: {"HighCard":<Rank>}
      | {"OnePair":<Rank>}
      | {"TwoPair":[<Rank> .. two]}
      | {"ThreeOfAKind":<Rank>}
      | {"Straight":<Rank>} // rank of the highest card
      | {"Flush":[<Rank> .. five]}
      | {"FullHouse":[<Rank> .. two]}
      | {"FourOfAKind":<Rank>}
      | {"StraightFlush":<Rank>} // rank of the highest card
{"General":<Message>}
Message: {"msg":{"PlayersInfo":[[<string>,<amount>] ..]}} // PlayersInfo is a list of pairs of player names and bank rolls and is sent at the beginning of each round..
	 | {"msg":{"Bet":{"player":<string>,"bet":<Bet>}}} // notification that a player placed a bet.
	 | {"msg":{"Round":<Stage>}} // notification of the stage.
	 | {"msg":{"RoundWinner":{"Winner":{"name":<string>,"hand":<Hand>,"cards":[<Card> .. seven]}}}} // notification that a player won a round.
	 | {"msg":{"Game":{"Winner":{"name":<string>,"hand":<Hand>,"cards":[<Card> .. seven]}}}} // notification that a player won the game.	
*/
import { FormEvent, useEffect, useState } from "react";

function App() {
  const [messages, setMessages] = useState<string[]>([]);
  const [socket, setSocket] = useState<WebSocket | undefined>(undefined);

  useEffect(() => {
    const socket = new WebSocket("ws://localhost:3000/");
    socket.onmessage = (e: MessageEvent<string>) =>
      setMessages((prev) => [...prev, e.data]);
    setSocket(socket);
    return () => socket.close();
  }, []);

  const submit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (!socket) return;
    const form = e.target as typeof e.target & {
      input: { value: string };
    };
    socket.send(form.input.value);
    form.input.value = "";
  };

  return (
    <>
      <h1>WebSocket Chat App</h1>
      <ul>
        {messages.map((body, index) => (
          <li key={index}>{body}</li>
        ))}
      </ul>
      <form onSubmit={submit}>
        <input type="text" name="input" />
        <button type="submit">Send</button>
      </form>
    </>
  );
}
export default App
