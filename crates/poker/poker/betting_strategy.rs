/// Betting strategies to be used by players.
use crate::poker::card::{Card, Hand, Rank};
use crate::poker::game::{Bet, Stage};

use super::compare::best_hand;
use super::sequence::same_suit;

#[derive(Debug, Clone)]
pub struct StrategyArgs {
    pub call: usize,
    pub min: usize,
    pub bank_roll: usize,
    pub cards: Vec<Card>,
    pub stage: Stage,
    pub cycle: u8,
}
/// Type for betting strategies.
pub type BettingStrategy = fn(StrategyArgs) -> Bet;

/// Default betting strategy, which will:
///
/// + fold if necessary,
/// + goes all in if neccessary,
/// + check if possible,
/// + call the bet.
pub fn default_betting_strategy(args: StrategyArgs) -> Bet {
    if args.bank_roll == 0 {
        Bet::Fold
    } else if args.bank_roll <= args.call {
        Bet::AllIn(args.bank_roll)
    } else if args.call == 0 {
        Bet::Check
    } else {
        Bet::Call
    }
}

/// Modest betting strategy, which will:
///
/// + fold if necessary,
/// + goes all in if neccessary,
/// + bet the minimum amount if currently zero,
/// + call the bet.
pub fn modest_betting_strategy(args: StrategyArgs) -> Bet {
    if args.bank_roll == 0 {
        Bet::Fold
    } else if args.bank_roll <= args.call {
        Bet::AllIn(args.bank_roll)
    } else if args.call == 0 {
        Bet::Raise(args.min)
    } else {
        Bet::Call
    }
}

/// A strategy that folds at the preflop for hands not in the top 15% of pairs of cards.
/// If we do have a good pair of hole cards, then raise twice in each betting stage, so
/// as we can afford it.
pub fn six_max(args: StrategyArgs) -> Bet {
    let mut cards = args.cards.clone();
    cards.sort();
    let hand = best_hand(&cards);
    let bet = std::cmp::min(args.bank_roll, args.call + args.min);
    let folding = args.bank_roll == 0;
    let all_in = args.bank_roll < args.call;
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
        let c1: Card = cards[0];
        let c2: Card = cards[1];
        let same_suit = same_suit(&cards);
        // if the hole cards are a pair, raise
        if let Hand::OnePair(..) = hand {
            make_bet(bet, folding, all_in, raising, args.cycle)
        } else {
            match c1.rank {
                Rank::Ace => {
                    if c2.rank > Rank::Rank10 || same_suit && c2.rank > Rank::Rank4 {
                        make_bet(bet, folding, all_in, raising, args.cycle)
                    } else {
                        Bet::Fold
                    }
                }
                Rank::King => {
                    if c2.rank > Rank::Rank10 || same_suit && c2.rank > Rank::Rank9 {
                        make_bet(bet, folding, all_in, raising, args.cycle)
                    } else {
                        Bet::Fold
                    }
                }
                Rank::Queen => {
                    if c2.rank > Rank::Rank10 {
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
