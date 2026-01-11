use crate::cards::deck::{Card, Deck, Suite, Value};
use crate::cards::hand::Hand;
use crate::game::choice::Choice;
use crate::game::choice::Choice::FIGHT_WITH_WEAPON;
use std::fmt::{Display, Formatter};
use std::io;
use std::io::Write;

#[derive(Debug)]
pub struct GameScore(Option<i32>);

pub struct Game {
    game_state: GameState,
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "===========================================================================\n"
        )
        .expect("Failed to write border to display");
        write!(f, "Health: {}\n", self.life).expect("Failed to write life points to display.");
        if let Some(weapon) = self.equipped_weapon.as_ref() {
            write!(f, "Equipped weapon: {weapon}\n")
                .expect("Failed to write equipped weapon to display.");
            write!(f, "Blocked creatures: {:?}\n", self.blocked_creatures)
                .expect("Failed to write blocked creatures to display");
        };
        write!(
            f,
            "===========================================================================\n"
        )
        .expect("Failed to write border to display");
        Ok(())
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            game_state: GameState::new(),
        }
    }

    /// returns the score at the end of the game
    pub fn start_game(&mut self) -> io::Result<GameScore> {
        let mut hand = Hand::new();

        loop {
            hand = self.game_state.draw_cards(hand, 4);

            /* check for game ending conditions */
            // if we can no longer draw 4 cards, the game ends
            if hand.num_cards_remaining() != 4 {
                return self.end_game(hand);
            }

            if self.game_state.life == 0 {
                return self.end_game(hand);
            }

            // do something with the input
            self.clear_screen();
            self.show_stats();
            self.show_hand(&hand);

            let mut choice = match self.read_user_input() {
                Ok(choice) => choice,
                Err(error) => {
                    println!("Error: {:?}", error);
                    continue; // continue to read user input until a valid input is received
                }
            };

            match choice {
                Choice::EXIT => {
                    return self.exit_game();
                }

                Choice::RUN => {
                    println!(
                        "You chose to run from the room! Shuffling cards back into the deck..."
                    );
                    self.game_state.put_back_cards(&mut hand);
                    continue;
                }

                Choice::OPTION(_) => {
                    loop {
                        match choice {
                            Choice::EXIT => {
                                return self.exit_game();
                            }

                            Choice::RUN => {
                                // get next choice
                                self.clear_screen();
                                self.show_stats();
                                self.show_hand(&hand);
                                println!("==> You cannot run from the room now.\n");

                                choice = match self.read_user_input() {
                                    Ok(choice) => choice,
                                    Err(error) => {
                                        println!("Error: {:?}", error);
                                        continue; // continue to read user input until a valid input is received
                                    }
                                }
                            }

                            Choice::OPTION(card_number) => {
                                match hand.remove_card(card_number as usize) {
                                    Some(card) => {
                                        println!("==> You chose {:?}", card);

                                        match card {
                                            Card {
                                                suite: Suite::Hearts,
                                                rank: _,
                                            } => {
                                                self.game_state.life = self
                                                    .game_state
                                                    .life
                                                    .saturating_add(card.rank.get_value() as u8);
                                            }
                                            Card {
                                                suite: Suite::Diamond,
                                                rank: _,
                                            } => {
                                                self.game_state.equipped_weapon = Some(card);
                                                self.game_state.blocked_creatures.clear(); // reset list of blocked creatures to None
                                            }
                                            Card {
                                                suite: Suite::Spade | Suite::Club,
                                                rank: _,
                                            } => {
                                                if let Some(weapon_card) =
                                                    self.game_state.equipped_weapon.as_ref()
                                                    && self
                                                        .game_state
                                                        .blocked_creatures
                                                        .last()
                                                        .map_or_else(
                                                            || true,
                                                            |last_blocked_creature| {
                                                                card.rank.get_value()
                                                                    < last_blocked_creature
                                                                        .rank
                                                                        .get_value()
                                                            },
                                                        )
                                                {
                                                    match self.read_user_input_for_combat_choice() {
                                                        Ok(choice) => {
                                                            match choice {
                                                                FIGHT_WITH_WEAPON(true) => {
                                                                    // fight with weapon
                                                                    let weapon_strength =
                                                                        weapon_card
                                                                            .rank
                                                                            .get_value();
                                                                    let creature_strength =
                                                                        card.rank.get_value();
                                                                    let damage_to_take =
                                                                        creature_strength
                                                                            .saturating_sub(
                                                                                weapon_strength,
                                                                            );

                                                                    // update life points
                                                                    self.game_state.life = self
                                                                        .game_state
                                                                        .life
                                                                        .saturating_sub(
                                                                            damage_to_take as u8,
                                                                        );

                                                                    // track list of creatures that were blocked
                                                                    self.game_state
                                                                        .blocked_creatures
                                                                        .push(card);
                                                                }
                                                                FIGHT_WITH_WEAPON(false) => {
                                                                    // bare-knuckle
                                                                    // TODO: eliminate duplicate (fight bare-knuckle)
                                                                    // update life points
                                                                    self.game_state.life = self
                                                                        .game_state
                                                                        .life
                                                                        .saturating_sub(
                                                                            card.rank.get_value()
                                                                                as u8,
                                                                        );
                                                                }
                                                                _ => {
                                                                    println!(
                                                                        "Please select enter either y/n"
                                                                    );
                                                                    continue; // continue to read user input until a valid input is received
                                                                }
                                                            }
                                                        }
                                                        Err(error) => {
                                                            println!("Error: {:?}", error);
                                                            continue; // continue to read user input until a valid input is received
                                                        }
                                                    }
                                                } else {
                                                    // bare-knuckle
                                                    // TODO: eliminate duplicate (fight bare-knuckle)
                                                    // update life points
                                                    self.game_state.life =
                                                        self.game_state.life.saturating_sub(
                                                            card.rank.get_value() as u8,
                                                        );
                                                }
                                            }
                                        }

                                        self.clear_screen();
                                        self.show_stats();
                                        self.show_hand(&hand);

                                        if self.game_state.life == 0 {
                                            return self.end_game(hand);
                                        }

                                        if hand.num_cards_remaining() > 1 {
                                            // get next choice
                                            choice = match self.read_user_input() {
                                                Ok(choice) => choice,
                                                Err(error) => {
                                                    println!("Error: {:?}", error);
                                                    continue; // continue to read user input until a valid input is received
                                                }
                                            };
                                        } else {
                                            break;
                                        }
                                    }
                                    None => {
                                        self.clear_screen();
                                        self.show_stats();
                                        self.show_hand(&hand);
                                        println!("==> You've already used this card");

                                        // get next choice
                                        choice = match self.read_user_input() {
                                            Ok(choice) => choice,
                                            Err(error) => {
                                                println!("Error: {:?}", error);
                                                continue; // continue to read user input until a valid input is received
                                            }
                                        };
                                    }
                                }
                            }
                            FIGHT_WITH_WEAPON(_) => {
                                println!("Invalid input!");
                                continue; // continue to read user input until a valid input is received
                            }
                        }
                    }
                }
                FIGHT_WITH_WEAPON(_) => {
                    println!("Invalid input!");
                    continue; // continue to read user input until a valid input is received
                }
            }
        }
    }

    fn read_user_input(&self) -> io::Result<Choice> {
        // show prompt to user
        println!("Enter the card number [1-4] to select it - to quit the game, enter q:");
        println!("If applicable, you may avoid the room by entering 0");

        // block for user input, until user hits enter
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        TryInto::<Choice>::try_into(input.as_ref())
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))
    }

    fn read_user_input_for_combat_choice(&self) -> io::Result<Choice> {
        // show prompt to user
        println!("Use weapon? [y/n]");

        // block for user input, until user hits enter
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        TryInto::<Choice>::try_into(input.as_ref())
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))
    }

    fn show_hand(&self, hand: &Hand) {
        println!("You drew these cards: ");
        println!("{hand}\n");
    }

    fn clear_screen(&self) {
        // TODO: use crossterm to clear the screen instead of this, as this might not work on Windows

        // ANSI: clear screen and move cursor to 1;1
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().ok();
    }

    fn show_stats(&self) {
        println!("{}", self.game_state);
    }

    fn end_game(&self, hand: Hand) -> io::Result<GameScore> {
        self.clear_screen();
        println!("Game ended!\n");
        if self.game_state.life == 0 {
            println!("You died!\n");
            let total_strength_of_monsters_left_in_deck = self
                .game_state
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
            Ok(GameScore(Some(-total_strength_of_monsters_left_in_deck)))
        } else {
            if hand.num_cards_remaining() == 1 {
                // TODO: we can simplify this for sure
                let bonus_score = hand
                    .iter()
                    .filter_map(|slot| slot.as_ref())
                    .filter(|card| card.suite == Suite::Hearts)
                    .map(|card| card.rank.get_value())
                    .take(1)
                    .fold(0, |total, curr| total + curr);

                let score = self.game_state.life as usize + bonus_score;
                Ok(GameScore(Some(score as i32)))
            } else {
                Ok(GameScore(Some(self.game_state.life as i32)))
            }
        }
    }

    fn exit_game(&self) -> io::Result<GameScore> {
        self.clear_screen();
        println!("Exiting the game...\n");
        Ok(GameScore(None)) // return a game score of None as the user has exited the game
    }
}

struct GameState {
    deck: Deck,
    life: u8,
    equipped_weapon: Option<Card>, // TODO: how can we make invalid states unrepresentable -> we should only be able to equip Diamond cards
    blocked_creatures: Vec<Card>, // TODO: how can we make invalid states unrepresentable -> we should only be able to store creature cards here (Club and Spade cards)
}

impl GameState {
    fn new() -> Self {
        GameState {
            deck: Deck::default(),
            life: 20,
            equipped_weapon: None,
            blocked_creatures: Vec::new(),
        }
    }

    fn draw_cards(&mut self, mut hand: Hand, hand_size: usize) -> Hand {
        let number_of_cards_to_draw = hand_size - hand.num_cards_remaining();

        for _ in 0..number_of_cards_to_draw {
            if let Some(card) = self.deck.draw_card() {
                hand.add_card(card);
            }
        }

        hand
    }

    fn put_back_cards(&mut self, hand: &mut Hand) {
        for slot in hand.iter_mut() {
            if let Some(card) = slot.take() {
                self.deck.insert_card(card);
            }
        }
    }
}
