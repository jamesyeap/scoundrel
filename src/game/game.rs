use crate::cards::deck::{Card, Deck};
use crate::game::choice::Choice;
use std::io;
use std::io::Write;

pub struct Game {
    game_state: GameState,
}

impl Game {
    pub fn new() -> Self {
        Self {
            game_state: GameState::new(),
        }
    }

    /// returns the score at the end of the game
    pub fn start_game(&mut self) -> io::Result<i16> {
        self.clear_screen();

        loop {
            let hand = self.game_state.draw_cards(4);

            println!("You drew: ");
            println!("{:?}", hand);

            // if we can no longer draw 4 cards, the game ends
            if hand.len() != 4 {
                // TODO: calculate a legit score and save it to GameState before breaking
                break;
            }

            let choice = match self.read_user_input() {
                Ok(choice) => choice,
                Err(error) => {
                    println!("Error: {:?}", error);
                    continue; // continue to read user input until a valid input is received
                }
            };

            // do something with the input
            match choice {
                // TODO: fetch the actual score from the GameState, and return it
                 Choice::EXIT => {
                    println!("Exiting the game...");
                    return Ok(-1); // return the user score
                }

                // TODO: do something with the chosen card
                Choice::OPTION(card_number) => println!("You chose card: {card_number}\n", ),
            }
        }

        Ok(-1)
    }

    fn read_user_input(&self) -> io::Result<Choice> {
        // show prompt to user
        println!("Enter the card number [1-4] to select it - to quit the game, enter 0:");

        // block for user input, until user hits enter
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        // parse the user input
        let result = input
            .trim()
            .parse::<u8>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "is not a valid u8"));

        self.clear_screen();

        TryInto::<Choice>::try_into(result?)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))
    }

    fn clear_screen(&self) {
        // TODO: use crossterm to clear the screen instead of this, as this might not work on Windows

        // ANSI: clear screen and move cursor to 1;1
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().ok();
    }
}

struct GameState {
    deck: Deck,
    life: u8,
}

impl GameState {
    fn new() -> Self {
        GameState {
            deck: Deck::default(),
            life: 20,
        }
    }

    fn draw_cards(&mut self, number_of_cards: usize) -> Vec<Card> {
        let mut hand = Vec::new();

        for _ in 0..number_of_cards {
            if let Some(card) = self.deck.draw_card() {
                hand.push(card);
            }
        }

        hand
    }

    fn put_back_cards(&mut self, cards: Vec<Card>) {
        for card in cards {
            self.deck.insert_card(card);
        }
    }
}
