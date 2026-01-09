use crate::cards::deck::{Card, Deck};
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

            // if we can no longer draw 4 cards, the game ends
            if hand.len() != 4 {
                // TODO: calculate a legit score and save it to GameState before breaking
                break;
            }

            let key = match self.read_user_input() {
                Ok(input) => input,
                Err(error) => {
                    println!("Error: {:?}", error);
                    continue; // continue to read user input until a valid input is received
                }
            };

            // do something with the input
            match key {
                // TODO: fetch the actual score from the GameState, and return it
                0 => {
                    println!("Exiting the game...");
                    return Ok(-1); // return the user score
                }

                // TODO: do something with the chosen card
                1..=4 => println!("You chose card: {key}\n"),

                // not possible to reach here, as we are already checking that
                // the user input is between 1 to 4 in read_user_input()
                _ => println!("Invalid key!"),
            }
        }

        Ok(-1)
    }

    fn read_user_input(&self) -> io::Result<u8> {
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

        if let Ok(key) = result
            && key > 4
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "input can only be 0, 1, 2, 3 or 4.",
            ));
        }

        self.clear_screen();

        result
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
