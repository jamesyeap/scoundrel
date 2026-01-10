use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::cards::deck::Card;

/// Represents a hand of cards that are drawn from the deck
#[derive(Debug)]
pub struct Hand {
    cards: HashMap<usize, Option<Card>>,
    num_cards: usize,
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (card_num, card) in self.cards.iter() {
            match card {
                Some(card) => write!(f, "[{:?}]: {card}\n", card_num).expect("Cannot print cards in hand to display."),
                None => write!(f, "CARD USED\n").expect("Cannot print cards in hand to display."),
            }
        }
        Ok(())
    }
}

impl Hand {
    pub fn new() -> Self {
        Self { cards: HashMap::new(), num_cards: 0 }
    }

    pub fn add_card(&mut self, card: Card) {
        let card_num = self.cards.len() + 1;
        self.cards.insert(card_num, Some(card));
        self.num_cards += 1;
    }

    pub fn remove_card(&mut self, card_num: usize) -> Option<Card> {
        if let Some(slot) = self.cards.get_mut(&card_num) {
            if let Some(card) = slot {
                self.num_cards -= 1;
                slot.take()
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn num_cards_remaining(&self) -> usize {
        self.num_cards
    }
}
