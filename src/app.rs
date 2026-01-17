use crate::cards::deck::{Card, Deck, Suite, Value};
use crate::cards::hand::Hand;

pub enum CurrentScreen {
    Menu,
    Exiting,

    BeforeRoom,
    ChooseCard,
    ChooseWeaponOrBareKnuckle,
    ExitingRound,
}

// holds the state
pub struct App {
    pub current_screen: CurrentScreen,

    pub deck: Deck,
    pub hand: Hand,
    pub life: u8,
    pub has_avoided_room: bool,

    pub equipped_weapon: Option<Card>, // should only hold diamond cards
    pub blocked_creatures: Vec<Card>,  // a stack
}

impl App {
    fn new() -> App {
        App {
            current_screen: CurrentScreen::BeforeRoom, // TODO: change this to CurrentScreen::Menu
            deck: Deck::default(),
            hand: Hand::new(),
            life: 0u8,
            has_avoided_room: false,
            equipped_weapon: None,
            blocked_creatures: Vec::new(),
        }
    }

    pub(crate) fn draw_cards(&mut self, hand_size: usize) {
        let number_of_cards_to_draw = hand_size - self.hand.num_cards_remaining();

        for _ in 0..number_of_cards_to_draw {
            if let Some(card) = self.deck.draw_card() {
                self.hand.add_card(card);
            }
        }
    }

    pub fn select_card(&mut self, card_idx: usize) -> Option<Card> {
        self.hand.remove_card(card_idx)
    }

    pub fn handle_card(&mut self, card: Card) {
        match card {
            Card {
                suite: Suite::Hearts,
                rank: _,
            } => {
                self.life = self.life.saturating_add(card.rank.get_value() as u8);
            }
            Card {
                suite: Suite::Diamond,
                rank: _,
            } => {
                self.equipped_weapon = Some(card);
                self.blocked_creatures.clear(); // reset list of blocked creatures to None
            }
            Card {
                suite: Suite::Spade | Suite::Club,
                rank: _,
            } => {
                if let Some(weapon_card) = self.equipped_weapon.as_ref()
                    && self.blocked_creatures.last().map_or_else(
                        || true,
                        |last_blocked_creature| {
                            card.rank.get_value() < last_blocked_creature.rank.get_value()
                        },
                    )
                {
                    // TODO:
                    //   figure out how to give user a choice
                } else {
                    // no suitable weapon equipped, only choice is to bare-knuckle:
                    // update life points
                    self.life = self.life.saturating_sub(card.rank.get_value() as u8);
                }
            }
        }
    }

    pub fn put_back_cards(&mut self) {
        // TODO: this is currently hardcoded to a handsize of 4 - refactor this
        for idx in 0..4 {
            let card_num = idx + 1;
            if let Some(card) = self.hand.remove_card(card_num) {
                self.deck.insert_card(card);
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}
