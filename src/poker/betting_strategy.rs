/// Betting strategies to be used by players.
use crate::poker::card::{Card, Hand, Rank};
use crate::poker::game::{Bet, Stage};

use super::compare::best_hand;
use super::sequence::same_suit;

/// Type for betting strategies.
pub type BettingStrategy = fn(usize, usize, usize, Vec<Card>, Stage, u8) -> Bet;

/// Default betting strategy, which will:
///
/// + fold if necessary,
/// + goes all in if neccessary,
/// + check if possible,
/// + call the bet.
pub fn default_betting_strategy(
    call: usize,
    _min: usize,
    bank_roll: usize,
    _cards: Vec<Card>,
    _stage: Stage,
    _cycle: u8,
) -> Bet {
    if bank_roll == 0 {
        Bet::Fold
    } else if bank_roll <= call {
        Bet::AllIn(bank_roll)
    } else if call == 0 {
        Bet::Check
    } else {
        Bet::Call(call)
    }
}

/// Modest betting strategy, which will:
///
/// + fold if necessary,
/// + goes all in if neccessary,
/// + bet the minimum amount if currently zero,
/// + call the bet.
pub fn modest_betting_strategy(
    call: usize,
    min: usize,
    bank_roll: usize,
    _cards: Vec<Card>,
    _stage: Stage,
    _cycle: u8,
) -> Bet {
    if bank_roll == 0 {
        Bet::Fold
    } else if bank_roll <= call {
        Bet::AllIn(bank_roll)
    } else if call == 0 {
        Bet::Raise(min)
    } else {
        Bet::Call(call)
    }
}

/// A strategy that folds at the preflop for hands not in the top 15% of pairs of cards.
/// If we do have a good pair of hole cards, then raise twice in each betting stage, so
/// as we can afford it.
pub fn six_max(
    call: usize,
    min: usize,
    bank_roll: usize,
    cards: Vec<Card>,
    stage: Stage,
    cycle: u8,
) -> Bet {
    let mut cards = cards.clone();
    cards.sort();
    let hand = best_hand(&cards);
    let bet = std::cmp::min(bank_roll, call + min);
    let folding = bank_roll == 0;
    let all_in = bank_roll < call;
    let raising = bet > call + min;
    fn make_bet(
        bet: usize,
        call: usize,
        folding: bool,
        all_in: bool,
        raising: bool,
        cycle: u8,
    ) -> Bet {
        if folding {
            Bet::Fold
        } else if all_in {
            Bet::AllIn(bet)
        } else if raising && cycle < 2 {
            // raise twice in a round
            Bet::Raise(bet)
        } else {
            Bet::Call(call)
        }
    }
    if let Stage::PreFlop = stage {
        let c1: Card = cards[0];
        let c2: Card = cards[1];
        let same_suit = same_suit(&cards);
        // if the hole cards are a pair, raise
        if let Hand::OnePair(..) = hand {
            make_bet(bet, call, folding, all_in, raising, cycle)
        } else {
            match c1.rank {
                Rank::Ace => {
                    if c2.rank > Rank::Rank10 || same_suit && c2.rank > Rank::Rank4 {
                        make_bet(bet, call, folding, all_in, raising, cycle)
                    } else {
                        Bet::Fold
                    }
                }
                Rank::King => {
                    if c2.rank > Rank::Rank10 || same_suit && c2.rank > Rank::Rank9 {
                        make_bet(bet, call, folding, all_in, raising, cycle)
                    } else {
                        Bet::Fold
                    }
                }
                Rank::Queen => {
                    if c2.rank > Rank::Rank10 {
                        make_bet(bet, call, folding, all_in, raising, cycle)
                    } else {
                        Bet::Fold
                    }
                }
                _ => Bet::Fold,
            }
        }
    } else {
        // if it's not PreFlop, make a bet
        make_bet(bet, call, folding, all_in, raising, cycle)
    }
}
