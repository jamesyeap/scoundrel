use std::io;
use std::io::Write;
use crate::cards::deck::{Deck, Card};

pub struct Game {
    game_state: GameState,
}

impl Game {
    pub fn new() -> Self {
        Self {
            game_state: GameState::new()
        }
    }

    /// returns the score at the end of the game
    pub fn start_game(&mut self) -> io::Result<i16> {
        loop {
            let hand = self.game_state.draw_cards(4);

            // if we can no longer draw 4 cards, the game ends
            if hand.len() != 4 {
                // TODO: calculate a legit score and save it to GameState before breaking
                break;
            }

            print!("Type something and press Enter: ");
            std::io::stdout().flush()?; // ensure prompt is shown
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?; // blocks until newline
            let input = input.trim_end();
            println!("You typed: {:?}", input);
        }

        // TODO: fetch the score from the GameState, and return it
        Ok(-1)
    }
}

struct GameState {
    deck: Deck,
    life: u8,
}

impl GameState {
    fn new() -> Self {
        GameState{
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