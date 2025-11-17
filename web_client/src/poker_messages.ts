// Types for the webapp.
export type ItemTuple = [string, number];

export type Rank = Rank2 | Rank3 | Rank4 | Rank5 | Rank6 | Rank7 | Rank8
    | Rank9 | Rank10 | Jack | Queen | King | Ace;

export type Suit = Clubs | Spades | Hearts | Diamonds;

export type Card = { rank: Rank; suit: Suit };

export type BetArgs = {
    call: number;
    community_cards: Card[];
    cycle: number;
    stage: string;
};

interface HighCard {
    type: 'HighCard';
    rank: Rank;
}

interface OnePair {
    type: 'OnePair';
    rank: Rank;
}

interface TwoPair {
    type: 'TwoPair';
    rank: Rank[];
}

interface ThreeOfAKind {
    type: 'ThreeOfAKind';
    rank: Rank;
}

interface Straight {
    type: 'Straight';
    rank: Rank;
}

interface Flush {
    type: 'Flush';
    rank: Rank[];
}

interface FullHouse {
    type: 'FullHouse';
    rank: Rank[];
}

interface FourOfAKind {
    type: 'FourOfAKind';
    rank: Rank;
}

interface StraightFlush {
    type: 'StraightFlush';
    rank: Rank;
}

export type Hand = HighCard | OnePair | TwoPair | ThreeOfAKind | Straight | Flush
    | FullHouse | FourOfAKind | StraightFlush;

interface Raise {
    type: 'Raise';
    amount: number;
}

interface AllIn {
    type: 'AllIn';
    amount: number;
}

interface Call {
    type: 'Call';
    amount: number;
}

export type Bet = 'Check' | Call | 'Fold' | Raise | AllIn;

// Message that another player placed a bet.
interface BetPlacedMessage {
    type: 'BetPlaced'; // Discriminator
    player: string;
    bet: Bet;
    pot: number;
}

// Players info message.
interface PlayersInfoMessage {
    type: 'PlayersInfo';
    players: ItemTuple[];
}

// Message for declaring the stage.
interface StageDeclMessage {
    type: 'StageDecl';
    stage: string;
    community_cards: Card[];
}

interface PlayerHand {
    type: 'PlayerHand';
    name: string;
    hand: Hand;
    cards: Card[];
}

interface SoleWinner {
    type: 'SoleWinner';
    winner: PlayerHand;
}

interface Draw {
    type: 'Draw';
    winners: PlayerHand[];
}

export type Winner = SoleWinner | Draw;

// Message for the winner of a round.
interface RoundWinnerMessage {
    type: 'RoundWinner';
    winner: Winner;
}

// Message for the winner of the game.
interface GameWinnerMessage {
    type: 'GameWinner';
    winner: Winner;
}

// Error Message.
interface ErrorMessage {
    type: 'Error'; // Discriminator
    message: string;
}

// Place Bet Request Message.
interface PlaceBetMessage {
    type: 'PlaceBet'; // Discriminator
    args: BetArgs;
    hole_cards: [Card, Card];
    bank_roll: number;
    best_hand: Hand;
}

// Player message.
interface Player {
    type: 'Player';
    name: string;
    bank_roll: number;
}

// Union Type for incoming messages.
export type IncomingPokerMessage = Player | PlaceBetMessage | BetPlacedMessage
    | PlayersInfoMessage | StageDeclMessage | RoundWinnerMessage
    | GameWinnerMessage | ErrorMessage;
