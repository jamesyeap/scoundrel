use crate::cards::deck::Card;

/// Represents a hand of cards that are drawn from the deck
#[derive(Debug)]
pub struct Hand<'a> {
    cards: Vec<&'a Card>,
}

impl<'a> Hand<'a> {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    pub fn add_card(&mut self, card: &'a Card) {
        self.cards.push(card);
    }

    pub fn remove_card_at_idx(&mut self, idx: usize) -> Option<&Card> {
        if idx > self.cards.len() - 1 {
            return None;
        }
        Some(self.cards.remove(idx))
    }
}
