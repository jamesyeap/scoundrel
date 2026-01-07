use std::hash::Hash;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use rand::prelude::*;
use rand::rng;

pub struct Deck {
    cards: Vec<Card>,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, EnumIter, Debug)]
enum Suite {
    Spade, Club, Diamond, Hearts,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, EnumIter, Debug)]
enum Rank {
    Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten,
    Jack, Queen, King,
    Ace,
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct Card {
    suite: Suite,
    rank: Rank,
}

impl Deck {
    fn new() -> Self {
        let mut cards = Vec::new();

        for suite in Suite::iter() {
            for rank in Rank::iter() {
                let card = Card{ suite, rank };
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
    use std::collections::HashSet;
    use crate::cards::deck::Deck;

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