use rand::prelude::*;
use rand::rng;
use std::fmt::{Display, Formatter, write};
use std::hash::Hash;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct Deck {
    cards: Vec<Card>,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, EnumIter, Debug)]
pub enum Suite {
    Spade,
    Club,
    Diamond,
    Hearts,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, EnumIter, Debug)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

pub trait Value {
    fn get_value(&self) -> usize;
}

impl Value for Rank {
    fn get_value(&self) -> usize {
        match self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct Card {
    pub suite: Suite,
    pub rank: Rank,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, {:?}", self.suite, self.rank).expect("Cannot print card to display.");
        Ok(())
    }
}

impl Deck {
    fn new() -> Self {
        let mut cards = Vec::new();

        for suite in Suite::iter() {
            for rank in Rank::iter() {
                let card = Card { suite, rank };
                cards.push(card);
            }
        }

        Self { cards }
    }

    pub fn draw_card(&mut self) -> Option<Card> {
        if self.cards.is_empty() {
            return None;
        }

        let mut rng = rng();

        // pick a card at random
        let num_cards_left = self.cards.len();
        let chosen_card_idx = rng.random_range(0..num_cards_left);
        let chosen_card = self.cards.remove(chosen_card_idx);

        Some(chosen_card)
    }

    pub fn insert_card(&mut self, card: Card) {
        // TODO: check if card already exists in deck before inserting it
        //         we should not have two copies of a card in a Deck
        self.cards.push(card);
    }
}

impl Default for Deck {
    fn default() -> Self {
        Deck::new()
    }
}

mod test {
    use crate::cards::deck::Deck;
    use std::collections::HashSet;

    #[test]
    fn cards_should_only_be_drawn_once() {
        let mut cards_drawn = HashSet::new();

        let mut deck = Deck::new();
        while let Some(card) = deck.draw_card() {
            assert!(!cards_drawn.contains(&card));
            cards_drawn.insert(card);
        }
    }
}
