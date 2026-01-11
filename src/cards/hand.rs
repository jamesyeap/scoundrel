use crate::cards::deck::Card;
use std::fmt::{Display, Formatter};
use std::slice::{Iter,IterMut};

/// Represents a hand of cards that are drawn from the deck
#[derive(Debug)]
pub struct Hand {
    cards: Vec<Option<Card>>,
    num_cards: usize,
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (idx, card) in self.cards.iter().enumerate() {
            let card_num = idx + 1;
            match card {
                Some(card) => write!(f, "[{:?}]: {card}\n", card_num)
                    .expect("Cannot print cards in hand to display."),
                None => write!(f, "[{:?}]: CARD USED\n", card_num)
                    .expect("Cannot print cards in hand to display."),
            }
        }
        Ok(())
    }
}

impl Hand {
    pub fn new() -> Self {
        Self {
            cards: Vec::new(),
            num_cards: 0,
        }
    }

    pub fn add_card(&mut self, card: Card) {
        if let Some(empty_slot) = self.cards.iter_mut().find(|slot| slot.is_none()) {
            *empty_slot = Some(card);
        } else {
            self.cards.push(Some(card));
        }
        self.num_cards += 1;
    }

    pub fn remove_card(&mut self, card_num: usize) -> Option<Card> {
        let idx = card_num - 1;
        match self.cards.get_mut(idx) {
            Some(slot) => {
                if slot.is_some() {
                    self.num_cards -= 1;
                }
                slot.take()
            }

            None => None,
        }
    }

    pub fn num_cards_remaining(&self) -> usize {
        self.num_cards
    }

    pub fn iter_mut(&'_ mut self) -> IterMut<'_, Option<Card>>  {
        self.cards.iter_mut()
    }

    pub fn iter(&'_ self) -> Iter<'_, Option<Card>>  {
        self.cards.iter()
    }
}