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
    borrow::Borrow, time::{Duration, SystemTime}, u128
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
        Suit::translate((gen_random() % 4) as u8)
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            0 => Suit::Heart,
            1 => Suit::Diamond,
            2 => Suit::Spade,
            3 => Suit::Club,
            _ => panic!("impossible"),
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        Rank::translate(((gen_random() % 14) + 1) as u8)
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


pub fn winner_card<T: Borrow<Card>>(card: T) -> bool {
    let card = card.borrow();
    *card
        == Card {
            suit: Suit::Spade,
            rank: Rank::Ace,
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let your_card = Card {
            rank: Rank::random(),
            suit: Suit::random(),
        };

        println!("Your card is {:?}", your_card);

        if winner_card(&your_card) {
            println!("You are the winner!");
        }

        if winner_card(your_card) {
            println!("You are the winner!");
        }
    }
}
