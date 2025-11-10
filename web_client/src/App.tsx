/**
Poker web app.
**/
import { FormEvent, useEffect, useState, useCallback } from "react";
import type { ItemTuple, Card, IncomingPokerMessage } from './poker_messages';
import Game from './Game';
import StartGame from './StartGame';

function App() {
  
  const [players, setPlayers] = useState<ItemTuple[]>([]);
  const [stage, setStage] = useState<string>("");
  const [player, setPlayer] = useState<Player | null>(null);
  const [bankRoll, setBankRoll] = useState<number>(0);
  const [socket, setSocket] = useState<WebSocket | undefined>(undefined);
  const [currentView, setCurrentView] = useState<string>('default');
  const [dealer, setDealer] = useState<string>('');
  const [holeCards, setHoleCards] = useState<string[]>([]);
  const [communityCards, setCommunityCards] = useState<string[]>([]);

  // State to track the connection status (Not connected, Connecting, Connected)
  const [connectionStatus, setConnectionStatus] = useState<'Disconnected' | 'Connecting' | 'Connected'>('Disconnected');

  const parseHoleCards = (cards: Card[]) => {
    const names = cards.map((c) => parseCard(c)); 
    setHoleCards(names);
  };

  const parseCommunityCards = (cards: Card[]) => {
    const names = cards.map((c) => parseCard(c)); 
    setCommunityCards(names);
  };

  const parseCard = (card: Card) => {
    const rank = card.rank.toLowerCase().replace('rank', '');
    return rank + '_of_' + card.suit.toLowerCase();
  };

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
	
      console.log('Message parsed as:');
      console.log(message);

      // Type narrowing using the discriminator.
      switch (message.type) {

        case 'Player':
	  const player = {
	    name: message.name,
	    bank_roll: message.bank_roll
          } as Player;
	  setPlayer(player);
	  break
	  
        case 'HoleCards':
	  parseHoleCards(message.cards);
	  break;
	  
        case 'PlaceBet':
	  console.log(`Bet request. Stage: ${message.args.stage}`);
	  parseHoleCards(message.hole_cards);
	  parseCommunityCards(message.args.community_cards);
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
	  setDealer(message.dealer);
	  setCurrentView('game');
	  break;

        case 'StageDecl':
	  console.log(`Stage: ${message.stage}`);
	  parseCommunityCards(message.community_cards);
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
    form.name.value = "";
    let request_players_msg = { RequestPlayers : true };
    socket.send(JSON.stringify(join_msg));
  };

  // The logic for conditional rendering
  const renderContent = () => {
    if (currentView === 'game') {
      // If state is 'game', render the Game view.
      return <Game player={player} players={players} dealer={dealer} holeCards={holeCards} communityCards={communityCards} />;
    } else {
      // Otherwise (if state is 'default'), render the StartGame view.
      return <StartGame submit={submit} />;
    }
  };

  return (
    <>
      <div className="container h-100">

      {renderContent()}
      
      </div>
    </>
  );
}
export default App
