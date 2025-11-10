/**
Poker web app.
**/
import { FormEvent, useEffect, useState } from "react";
//import { Card, IncomingPokerMessage } from './poker_messages';
import * as PM from './poker_messages';

type ItemTuple = [string, number];

function App() {
  
  const [players, setPlayers] = useState<ItemTuple[]>([]);
  const [stage, setStage] = useState<string>("");
  const [playerName, setPlayerName] = useState<string>("");
  const [bankRoll, setBankRoll] = useState<number>(0);
  const [socket, setSocket] = useState<WebSocket | undefined>(undefined);

  useEffect(() => {
    const socket = new WebSocket("ws://127.0.0.1:3000/");
    socket.onmessage = (e: MessageEvent<string>) =>
      handleMessage(e.data);
    setSocket(socket);
  }, []);

  const handleMessage = (msg: object) => {
    try {
      const rawMessage = JSON.parse(event.data);
      // map the incoming JSON key into our union type.
      const typeKey = Object.keys(rawMessage)[0];
      const message: PM.IncomingPokerMessage = {
        type: typeKey as PM.IncomingPokerMessage['type'],
	...rawMessage[typeKey] // spread the inner payload
      };

      console.log(message);

      // Type narrowing using the discriminator.
      switch (message.type) {
        case 'General':
	  console.log(`General update: ${message.msg}`);
	  break;

        case 'PlaceBet':
	  console.log(`Bet request. Pot size: ${message.args.stage}`);
	  // send a bet back to the server.
	  break;

        case 'Error':
	  console.error(`SERVER ERROR: ${message.msg}`);
	  break;

        default:
	  // Fallback for malformed or unexpected messages.
	  console.warn('Received unknown message format: ', message);
	  break;
      }
      
    } catch (error) {
      console.log('Received unknown message format: ', error);
    }
  }

  const submit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (!socket) return;
    const form = e.target as typeof e.target & {
      name: { value: string };
    };
    let join_msg = { NewGame : { name : form.name.value } };
    socket.send(JSON.stringify(join_msg));
    setPlayerName(e.data);
    form.name.value = "";
  };

  return (
    <>
      <h1>Start a game</h1>
      <form onSubmit={submit}>
        <label htmlFor="name">Name</label>
	<input type="text" name="name" id="name" />
        <button type="submit">Start game</button>
      </form>
    </>
  );
}
export default App
