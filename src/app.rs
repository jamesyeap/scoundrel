use crate::cards::deck::{Card, Deck, Suite, Value};
use crate::cards::hand::Hand;
use crate::game::game::GameScore;
use color_eyre::eyre;
use color_eyre::eyre::bail;

// CONSTANTS
const MAX_LIFE: u8 = 20;
pub const HAND_SIZE: usize = 4;

#[derive(Eq, PartialEq)]
pub enum CurrentScreen {
    Welcome,

    Menu,
    Exiting,

    BeforeRoom,
    ChooseCard,
    ChooseWeaponOrBareKnuckle,
    ExitingRound,

    Won,
    Lost,
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
    pub in_combat_with_creature: Option<Card>, // holds the creature that the user is currently attacking - used when user is choosing whether to fight with weapon, or bare-knuckle

    pub notifications: Vec<String>, // info or alert messages to display to the user
}

impl App {
    fn new() -> App {
        App {
            current_screen: CurrentScreen::Welcome, // TODO: change this to CurrentScreen::Menu
            deck: Deck::default(),
            hand: Hand::new(),
            life: 20u8,
            has_avoided_room: false,
            equipped_weapon: None,
            blocked_creatures: Vec::new(),
            in_combat_with_creature: None,
            notifications: Vec::new(),
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

    /// Respond to the card that the user has chosen, and returns the next screen to display (if necessary):
    /// if no screen is returned, then the next screen as defined by the state transition will be shown.
    pub fn handle_card(&mut self, card: Card) -> color_eyre::Result<Option<CurrentScreen>> {
        match card {
            Card {
                suite: Suite::Heart,
                rank: _,
            } => self.add_to_life(card),
            Card {
                suite: Suite::Diamond,
                rank: _,
            } => self.equip_weapon(card),

            Card {
                suite: Suite::Spade | Suite::Club,
                rank: _,
            } => self.fight_creature(card),
        }
    }

    pub fn put_back_cards(&mut self) {
        for idx in 0..HAND_SIZE {
            let card_num = idx + 1;
            if let Some(card) = self.hand.remove_card(card_num) {
                self.deck.insert_card(card);
            }
        }
    }

    /// Adds points from Heart cards to player's life points, up to a maximum of 20 points
    fn add_to_life(&mut self, card: Card) -> eyre::Result<Option<CurrentScreen>> {
        if card.suite != Suite::Heart {
            bail!("Only cards with Heart suite can be used to add life points!");
        }

        let life_points_given_by_card: u8 = card.rank.get_value().try_into()?;
        let life_points_addable: u8 = MAX_LIFE - self.life; // user's life cannot exceed MAX_LIFE points
        let life_points_to_add = std::cmp::min(life_points_given_by_card, life_points_addable);
        self.life = self.life.saturating_add(life_points_to_add);

        // display notification
        self.notifications
            .push(format!("You gained {life_points_to_add} life points"));

        Ok(None)
    }

    fn equip_weapon(&mut self, card: Card) -> eyre::Result<Option<CurrentScreen>> {
        self.equipped_weapon = Some(card);
        self.blocked_creatures.clear(); // reset list of blocked creatures to None

        // display notification
        self.notifications.push(format!(
            "You equipped: {}",
            self.equipped_weapon.as_ref().unwrap()
        ));

        Ok(None)
    }

    fn fight_creature(&mut self, card: Card) -> eyre::Result<Option<CurrentScreen>> {
        if self.equipped_weapon.is_some()
            && self.blocked_creatures.last().map_or_else(
                || true,
                |last_blocked_creature| {
                    card.rank.get_value() < last_blocked_creature.rank.get_value()
                },
            )
        {
            // TODO: remove duplicate
            self.in_combat_with_creature = Some(card);

            // give user a choice whether to use the weapon, or to bare-knuckle
            Ok(Some(CurrentScreen::ChooseWeaponOrBareKnuckle))
        } else {
            // TODO: remove duplicate
            self.in_combat_with_creature = Some(card);
            self.fight_creature_bare_knuckle()
        }
    }

    pub fn fight_creature_with_weapon(&mut self) -> eyre::Result<Option<CurrentScreen>> {
        // fight with weapon
        let weapon_strength = self.equipped_weapon.as_ref().unwrap().rank.get_value();

        let creature = self.in_combat_with_creature.take().unwrap();
        let creature_strength = creature.rank.get_value();
        let damage_to_take = creature_strength.saturating_sub(weapon_strength);

        // update life points
        self.life = self.life.saturating_sub(damage_to_take as u8);

        // display notification
        self.add_notification(format!("You took {damage_to_take} damage"));

        if self.life == 0 {
            // if life points remaining after this encounter is 0, player has lost the game
            Ok(Some(CurrentScreen::Lost))
        } else {
            // track list of creatures that were blocked
            self.blocked_creatures.push(creature);

            Ok(None)
        }
    }

    pub fn fight_creature_bare_knuckle(&mut self) -> eyre::Result<Option<CurrentScreen>> {
        // no suitable weapon equipped, only choice is to bare-knuckle:
        let creature = self.in_combat_with_creature.take().unwrap();
        let damage_to_take = creature.rank.get_value() as u8;

        // subtract life points
        self.life = self.life.saturating_sub(damage_to_take);

        // display notification
        self.add_notification(format!("You took {damage_to_take} damage"));

        if self.life == 0 {
            // if life points remaining after this encounter is 0, player has lost the game
            Ok(Some(CurrentScreen::Lost))
        } else {
            Ok(None)
        }
    }

    // used to calculate score at the end of the game
    pub fn calculate_score(&self) -> i32 {
        if self.life > 0 {
            let score = if self.hand.num_cards_remaining() == 1 {
                let bonus_score = self
                    .hand
                    .iter()
                    .filter_map(|slot| slot.as_ref())
                    .find(|card| card.suite == Suite::Heart)
                    .map_or_else(|| 0, |card| card.rank.get_value());

                self.life + (bonus_score as u8)
            } else {
                self.life
            };

            score as i32
        } else {
            let total_strength_of_monsters_left_in_deck = self
                .deck
                .iter()
                .map(|card| {
                    match card {
                        // if the card is a monster, add its strength
                        Card {
                            suite: Suite::Spade | Suite::Club,
                            rank: _,
                        } => card.rank.get_value() as i32,
                        _ => 0,
                    }
                })
                .fold(0, |total, elem| total + elem);

            -total_strength_of_monsters_left_in_deck
        }
    }

    fn add_notification(&mut self, notification: String) {
        self.notifications.push(notification);
    }
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}
