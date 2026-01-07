mod cards;
mod game;

fn main() {
    let mut game = game::game::Game::new();
    
    let score = game.start_game();
    if let Ok(score) = score {
        println!("Your score: {}", score);
    }
}
