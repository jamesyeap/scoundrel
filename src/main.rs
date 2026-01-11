//! Play Scoundrel on your CLI.

mod cards;
mod game;

fn main() {
    let mut game = game::game::Game::new();
    
    let score = game.start_game();
    if let Ok(game_score) = score {
        println!("Your score: {:?}", game_score);
    }
}
