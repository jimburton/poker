// Types for the webapp.
export type Card = { suit: string; rank: string };
export type Stage = 'PreFlop' | 'Flop' | 'Turn' | 'River' | 'ShowDown';
export type BetArgs = { call: number; community_cards: Card[]; cycle: number; stage: Stage };
export type Hand = { rank: string };
export type Msg = 'PlayerTurn' | 'GameStart' | 'RoundEnd';

// General Update Message.
interface GeneralMessage {
    type: 'General'; // Discriminator
    msg: Msg; // The contents of the Msg enum from Rust
}

// Place Bet Request Message.
interface PlaceBetMessage {
    type: 'PlaceBet'; // Discriminator
    args: BetArgs;
    hole_cards: [Card, Card];
    bank_roll: number;
    best_hand: Hand;
}

// Error Message.
interface ErrorMessage {
    type: 'Error'; // Discriminator
    message: string;
}

// The Union Type for incoming messages.
export type IncomingPokerMessage = GeneralMessage | PlaceBetMessage | ErrorMessage;

