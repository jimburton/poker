/**
Poker web app.
**/
import { FormEvent, useEffect, useState, useCallback } from "react";
import type { ItemTuple, Card, IncomingPokerMessage } from './poker_messages';
import GameView from './components/GameView';
import StartGame from './components/StartGame';

function App() {
  
  const [players, setPlayers] = useState<ItemTuple[]>([]);
  const [stage, setStage] = useState<string>("");
  const [player, setPlayer] = useState<Player | null>(null);
  const [playerName, setPlayerName] = useState<string>('');
  const [bankRoll, setBankRoll] = useState<number>(0);
  const [socket, setSocket] = useState<WebSocket | undefined>(undefined);
  const [currentView, setCurrentView] = useState<string>('default');
  const [dealer, setDealer] = useState<string>('');
  const [holeCards, setHoleCards] = useState<string[]>([]);
  const [communityCards, setCommunityCards] = useState<string[]>([]);
  const [possibleBets, setPossibleBets] = useState<string[]>(['Fold']);
  const [bestHandCards, setBestHandCards] = useState<string[]>([]);
  const [call, setCall] = useState<number>(0);
  const [minBet, setMinBet] = useState<number>(0);
  const [pot, setPot] = useState<number>(0);
  const [messageQueue, setMessageQueue] = useState([]);

  // State to track the connection status (Not connected, Connecting, Connected)
  const [connectionStatus, setConnectionStatus] = useState<'Disconnected' | 'Connecting' | 'Connected'>('Disconnected');

  const enqueueMessage = useCallback((text) => {
    if (text.trim()) {
      const id = Math.random().toString(36).slice(2);
      console.log(`id: ${id}`);
      const msg = {
        id: id,
        text: text,
        duration: 30000,
       };
      console.log('Enqueueing message:');
      console.log(msg);
      setMessageQueue(prevQueue => [...prevQueue, msg]);
      console.log(`Messages: ${messageQueue}`);
    }
  }, [setMessageQueue]);

  const parseHoleCards = (cards: Card[]) => {
    const names = cards.map((c) => parseCard(c)); 
    setHoleCards(names);
  };

  const parseCommunityCards = (cards: Card[]) => {
    const names = cards.map((c) => parseCard(c)); 
    setCommunityCards(names);
  };

  const parseBestHandCards = (cards: Card[]) => {
    const names = cards.map((c) => parseCard(c)); 
    setBestHandCards(names);
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
      let msgStr = '';
      
      // Type narrowing using the discriminator.
      switch (message.type) {

        case 'Player':
	  setPlayerName(message.name);
	  setBankRoll(message.bank_roll);
	  break
	  
        case 'HoleCards':
	  parseHoleCards(message.cards);
	  break;
	  
        case 'PlaceBet':
	  console.log(`Bet request. Stage: ${message.args.stage}`);
	  console.log(message);
	  parseHoleCards(message.hole_cards);
	  parseCommunityCards(message.args.community_cards);
	  parseBestHandCards(message.best_hand);
	  console.log("Best hand cards:");
	  console.log(message.best_hand.cards);
	  setBankRoll(message.bank_roll);
	  setCall(message.args.call);
	  setMinBet(message.args.min);
	  let bets: string[] = ['Fold'];
	  if (message.bank_roll > 0) {
            bets.push('AllIn');
          }
	  if (message.args.call === 0) {
            bets.push('Check');
          }
	  if (message.args.call < bankRoll) {
            bets.push('Call')
          }
	  if (message.args.call + message.args.min < message.bank_roll) {
            bets.push('Raise');
          }
	  setPossibleBets(bets);
	  enqueueMessage("Time to place a bet");
	  // player needs to send bet back to the server
	  break;

        case 'BetPlaced':
	  msgStr = `${message.player} made bet ${formatBet(message.bet)}`;
	  console.log(msgStr);
	  enqueueMessage(msgStr);
	  setPot(parseInt(message.pot));
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
	  msgStr = `Stage: ${message.stage}`;
	  console.log(msgStr);
	  parseCommunityCards(message.community_cards);
	  enqueueMessage(msgStr);
	  break;

        case 'RoundWinner':
	  const winnerType = Object.keys(message.winner)[0];
	  const winner = message.winner[winnerType];
	  if (winnerType === 'SoleWinner') {
	    msgStr = `${winner.name} won the round with ${JSON.stringify(winner.hand)}`;
	    console.log(msgStr);
	    enqueueMessage(msgStr);
          } else {
	    let winners = winner.map((w) => w[0]).join(', ');
	    msgStr = `The round was a draw between ${winners}`;
	    console.log(msgStr);
	    enqueueMessage(msgStr);
          }
	  break;

        case 'GameWinner':
	  if (message.winner.type === 'SoleWinner') {
	    msgStr = `${message.winner.name} won the game.`;
	    console.log(msgStr);
	    enqueueMessage(msgStr);
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
  }, [setConnectionStatus, setPlayers, setBankRoll, setCall, setPlayerName,
      setPossibleBets, setMinBet, setCurrentView, setBestHand, setHoleCards,
      setCommunityCards, setDealer]);

  const formatBet = (bet) => {
    
    if (bet === 'Fold' || bet === 'Check' || bet === 'Call') {
      return bet;
    } else {
      let payload = bet['PlayerBet'];
      let type = Object.keys(payload)[0];
      let amount = payload[type];
      return `${type} (${amount})`;
    }
  };
  
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

  const placeBet = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (!socket) return;

    console.log(e);
    let btn = e.nativeEvent.submitter.name;
    console.log(btn);

    const form = e.target as typeof e.target & {
      amount: { value: number };
    };

    let bet;
    switch (btn) {
      case 'Fold':
        bet = {
          PlayerBet: 'Fold'
        };
        break;
      case 'Check':
        bet = {
	  PlayerBet: 'Check'
        };
        break;
      case 'Call':
        bet = {
	  PlayerBet: {Call: parseInt(form.amount.value)}
        };
	setPot(pot + parseInt(form.amount.value));
	setBankRoll(bankRoll-form.amount.value);
        break;
	
      case 'Raise':
        bet = {
	  PlayerBet: {Raise: parseInt(form.amount.value)}
        };
	setBankRoll(bankRoll-form.amount.value);
	setPot(pot + parseInt(form.amount.value));
        break;
	
      case 'AllIn':
        bet = {
	  PlayerBet: {AllIn: parseInt(form.amount.value)}
        };
	setBankRoll(bankRoll-form.amount.value);
	setPot(pot + parseInt(form.amount.value));
        break;
    }
    console.log(bet);
    enqueueMessage(`You made bet ${formatBet(bet)}`);
    socket.send(JSON.stringify(bet));
  };

  // The logic for conditional rendering
  const renderContent = () => {
    if (currentView === 'game') {
      // If state is 'game', render the Game view.
      return <GameView playerName={playerName} bankRoll={bankRoll}
                   players={players} dealer={dealer} holeCards={holeCards}
		   communityCards={communityCards} possibleBets={possibleBets}
		   bestHand={bestHand} call={call} minBet={minBet} placeBet={placeBet}
		   pot={pot} messageQueue={messageQueue} setMinBet={setMinBet} />;
    } else {
      // Otherwise (if state is 'default'), render the StartGame view.
      return <StartGame submit={submit} />;
    }
  };

  return (
  
      <div className="container h-100">

      {renderContent()}
      
      </div>
  );
}
export default App
