/// Betting strategies to be used by players.
use crate::poker::{
    card::{Card, Hand, Rank},
    compare,
    game::{Bet, Stage},
    sequence,
};
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Struct for arguments to place_bet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetArgs {
    pub call: usize,
    pub min: usize,
    pub stage: Stage,
    pub cycle: u8,
    pub community_cards: Vec<Card>,
}
/// Type for betting strategies.
pub type BettingStrategy = fn(BetArgs, (Card, Card), usize) -> Bet;

/// Default betting strategy, which will:
///
/// + fold if necessary,
/// + goes all in if neccessary,
/// + check if possible,
/// + call the bet.
pub fn default_betting_strategy(args: BetArgs, _hole_cards: (Card, Card), bank_roll: usize) -> Bet {
    if bank_roll == 0 {
        Bet::Fold
    } else if bank_roll <= args.call {
        Bet::AllIn(bank_roll)
    } else if args.call == 0 {
        Bet::Check
    } else {
        Bet::Call
    }
}

/// Modest betting strategy, which will:
///
/// + fold if necessary,
/// + go all in if neccessary,
/// + toss a coin to choose between betting some value not more than twice
///   the minimum amount and calling the bet.
pub fn modest_betting_strategy(args: BetArgs, _hole_cards: (Card, Card), bank_roll: usize) -> Bet {
    if bank_roll == 0 {
        Bet::Fold
    } else if bank_roll <= args.call {
        Bet::AllIn(bank_roll)
    } else {
        // toss a coin between raising and calling.
        if rand::random() {
            // choose a value between min and min*2 or one chip less than bank_roll
            // , whichever is lower.
            let max = std::cmp::min(args.min * 2, bank_roll - 1);
            let mut rng = rand::rng();
            let amount = rng.random_range(args.min..max);
            Bet::Raise(amount)
        } else {
            Bet::Call
        }
    }
}

/// A strategy that folds at the preflop for hands not in the top 15% of pairs of cards.
/// If we do have a good pair of hole cards, then raise twice in each betting stage, so
/// as we can afford it.
pub fn six_max(args: BetArgs, hole_cards: (Card, Card), bank_roll: usize) -> Bet {
    let mut cards = args.community_cards.clone();
    cards.push(hole_cards.0);
    cards.push(hole_cards.1);
    cards.sort();
    let hand = compare::best_hand(&cards);
    let bet = std::cmp::min(bank_roll, args.call + args.min);
    let folding = bank_roll == 0;
    let all_in = bank_roll < args.call;
    let raising = bet > args.call + args.min;
    fn make_bet(bet: usize, folding: bool, all_in: bool, raising: bool, cycle: u8) -> Bet {
        if folding {
            Bet::Fold
        } else if all_in {
            Bet::AllIn(bet)
        } else if raising && cycle < 2 {
            // raise twice in a round
            Bet::Raise(bet)
        } else {
            Bet::Call
        }
    }
    if let Stage::PreFlop = args.stage {
        // the only cards in cards are the hole cards.
        let same_suit = sequence::same_suit(&cards);
        // if the hole cards are a pair, raise.
        if let Hand::OnePair(..) = hand.hand {
            make_bet(bet, folding, all_in, raising, args.cycle)
        } else {
            let (h1, h2) = (hole_cards.0, hole_cards.1);
            match h1.rank {
                Rank::Ace => {
                    if h2.rank > Rank::Rank10 || same_suit && h2.rank > Rank::Rank4 {
                        make_bet(bet, folding, all_in, raising, args.cycle)
                    } else {
                        Bet::Fold
                    }
                }
                Rank::King => {
                    if h2.rank > Rank::Rank10 || same_suit && h2.rank > Rank::Rank9 {
                        make_bet(bet, folding, all_in, raising, args.cycle)
                    } else {
                        Bet::Fold
                    }
                }
                Rank::Queen => {
                    if h2.rank > Rank::Rank10 {
                        make_bet(bet, folding, all_in, raising, args.cycle)
                    } else {
                        Bet::Fold
                    }
                }
                _ => Bet::Fold,
            }
        }
    } else {
        // if it's not PreFlop, make a bet
        make_bet(bet, folding, all_in, raising, args.cycle)
    }
}
