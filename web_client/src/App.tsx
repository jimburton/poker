/**
Poker web app.
**/
import { FormEvent, useEffect, useState, useCallback } from "react";
import type { ItemTuple, Card, IncomingPokerMessage } from './poker_messages';

function App() {
  
  const [players, setPlayers] = useState<ItemTuple[]>([]);
  const [stage, setStage] = useState<string>("");
  const [playerName, setPlayerName] = useState<string>("");
  const [bankRoll, setBankRoll] = useState<number>(0);
  const [socket, setSocket] = useState<WebSocket | undefined>(undefined);

  // State to track the connection status (Not connected, Connecting, Connected)
  const [connectionStatus, setConnectionStatus] = useState<'Disconnected' | 'Connecting' | 'Connected'>('Disconnected');

  const handleMessage = useCallback((data: string) => {
    if (connectionStatus !== 'Connected') {
      setConnectionStatus('Connected');
    }
    try {
      console.log(`Unparsed: ${data}`);
      const rawMessage = JSON.parse(data);
      // map the incoming JSON key into our union type.
      const typeKey = Object.keys(rawMessage)[0];
      const payload = rawMessage[typeKey];
      
      console.log(`Incoming message type: ${typeKey}`);

      const message = {
          type: typeKey as IncomingPokerMessage['type'],
	  ...(payload as object) // spread the inner payload
        } as IncomingPokerMessage;
	
      /*let message: IncomingPokerMessage;

      if (typeKey === 'PlayersInfo' && Array.isArray(payload)) {
        message = {
          type: 'PlayersInfo',
	  players: payload as ItemTuple[]
        } as IncomingPokerMessage;
      } else if (typeKey === 'StageDecl') {
        message = {
          type: 'StageDecl',
	  stage: payload as string
        } as IncomingPokerMessage;
      } else {
        message = {
          type: typeKey as IncomingPokerMessage['type'],
	  ...(payload as object) // spread the inner payload
        } as IncomingPokerMessage;
      }*/

      console.log('Message parsed as:');
      console.log(message);

      // Type narrowing using the discriminator.
      switch (message.type) {
        case 'PlaceBet':
	  console.log(`Bet request. Stage: ${message.args.stage}`);
	  // player needs to send bet back to the server
	  break;

        case 'BetPlaced':
	  console.log(`Player ${message.player} made bet ${message.bet}`);
	  break;

        case 'PlayersInfo':
	  setPlayers(message.players);
	  let playersStr = message.players.map((p) => p[0] + ' (' + p[1] + ')').join(", ");
	  console.log(`Players: ${playersStr}`);
	  // set up the UI.
	  break;

        case 'StageDecl':
	  console.log(`Stage: ${message.stage}`);
	  break;

        case 'RoundWinner':
	  if (message.winner.type === 'SoleWinner') {
	    console.log(`${message.winner.name} won the round with ${message.winner.hand}`);
          } else {
	    let winners = message.winners.map((w) => w[0]).join(', '); 
	    console.log(`It was a draw between ${winners}`);
          }
	  break;

        case 'GameWinner':
	  if (message.winner.type === 'SoleWinner') {
	    console.log(`${message.winner.name} won the game.`);
          } else {
	    console.error(`Was expecting a single winner of the game, got ${message}`);
          }
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
  }, [setConnectionStatus, setPlayers]);
  
  useEffect(() => {
    console.log('Connecting to server.');
    setConnectionStatus('Connecting');
    const ws = new WebSocket("ws://127.0.0.1:3000/");

    ws.onopen = () => {
      setConnectionStatus('Connected');
      setSocket(ws); // Store the live instance
      console.log("WebSocket connection established (OPEN).");
    };

    ws.onmessage = (event) => {
      // event.data contains the message payload (usually a JSON string)
      handleMessage(event.data);
    };
        
    ws.onclose = (event) => {
      setConnectionStatus('Disconnected');
      setSocket(null); // Clear the instance
      console.log(`WebSocket connection closed. Code: ${event.code}, Reason: ${event.reason}`);
    };

    // Fired if there is an unrecoverable error
    ws.onerror = (error) => {
      setConnectionStatus('Disconnected');
      console.error("WebSocket Error:", error);
    };
  }, [handleMessage, setSocket, setConnectionStatus]);

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
