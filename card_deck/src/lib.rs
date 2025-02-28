// A standard deck of cards has 52 cards: 4 suits with 13 cards per suit. Represent the cards from a deck:
//
//     Create an enum to represent the Suit.
//     Implement the associated function random, which returns a random Suit (Heart, Diamond, Spade or Club).
//     Create a Rank enum. For ace and face cards, it can be one of Ace, King, Queen or Jack. For the values from 2 to 10, it can have a Number value associated to a u8.
//     Create an associated function to Rank called Random that returns a random Rank.
//     Create a structure name Card which has the fields suit and rank.
//
// Define:
//
//     The associated function translate for Rank and Suit:
//         For Suit, translate converts an integer value (u8) to a suit (1 -> Heart, 2 -> Diamonds, 3 -> Spade, 4 -> Club).
//         For Rank, translate converts an integer value (u8) to a rank ( 1 -> Ace, 2 -> 2, .., 10 -> 10, 11 -> Jack, 12 -> Queen, 13 -> King).
//     The associated function random for Rank and Suit returns a random Rank and Suit respectively.
//     Finally define the function winner_card which returns true if the card passed as an argument is an ace of spades.
//

use core::panic;
use std::{
    time::{Duration, SystemTime},
    u128,
};

fn gen_random() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_nanos()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        Suit::translate(((gen_random() % 4) + 1) as u8)
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("impossible"),
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        Rank::translate(((gen_random() % 13) + 1) as u8)
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            x @ 2..=10 => Rank::Number(x),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => panic!("impossible"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: Card) -> bool {
    card == Card {
        suit: Suit::Spade,
        rank: Rank::Ace,
    }
}

pub fn gen_card() -> Card {
    Card {
        rank: Rank::random(),
        suit: Suit::random(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        loop {
            let card = gen_card();
            println!("draw card: {card:#?}");
            if winner_card(card) {
                println!("You are the winner!");
                break;
            }
        }
    }
}
