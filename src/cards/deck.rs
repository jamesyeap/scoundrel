use rand::prelude::*;
use rand::rng;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::slice::Iter;
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
    Heart,
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
        // human-friendly short rank
        let rank_str = match self.rank {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        };

        // base code point per suit in the Unicode Playing Cards block
        let base = match self.suite {
            Suite::Spade => 0x1F0A0,
            Suite::Heart => 0x1F0B0,
            Suite::Diamond => 0x1F0C0,
            Suite::Club => 0x1F0D0,
        };

        // offsets for ranks in the block. Note: the 'knight' (0xC) exists in Unicode but
        // not in a standard 52-card deck, so we skip that slot for queen/king.
        let offset = match self.rank {
            Rank::Ace => 0x1,
            Rank::Two => 0x2,
            Rank::Three => 0x3,
            Rank::Four => 0x4,
            Rank::Five => 0x5,
            Rank::Six => 0x6,
            Rank::Seven => 0x7,
            Rank::Eight => 0x8,
            Rank::Nine => 0x9,
            Rank::Ten => 0xA,
            Rank::Jack => 0xB,
            // 0xC is the 'Knight' (tarot); skip it
            Rank::Queen => 0xD,
            Rank::King => 0xE,
        };

        let codepoint = base + offset;
        let glyph = std::char::from_u32(codepoint).unwrap_or('\u{FFFD}');

        let suite_name = match self.suite {
            Suite::Spade => "spades",
            Suite::Heart => "hearts",
            Suite::Diamond => "diamonds",
            Suite::Club => "clubs",
        };

        // write!(f, "{} of {} {}", rank_str, suite_name, glyph)
        // write!(f, "{glyph}")
        // write!(f, "🂢")
        write!(f, "<{:?}, {:?}>", self.suite, self.rank).expect("Cannot print card to display.");

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

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn iter(&'_ self) -> Iter<'_, Card> {
        self.cards.iter()
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
