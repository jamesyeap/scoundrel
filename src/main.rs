//! Play Scoundrel on your CLI.

extern crate core;

use crate::app::{App, CurrentScreen};
use crate::ui::ui;
use crossterm::event::KeyEventKind::Press;
use crossterm::event::{Event, KeyCode, read};
use ratatui::DefaultTerminal;

mod app;
mod cards;
mod game;
mod ui;

fn main() -> color_eyre::Result<()> {
    // let mut game = game::game::Game::new();
    //
    // let score = game.start_game();
    // if let Ok(game_score) = score {
    //     println!("{game_score}");
    // }

    ratatui::run(start)?;
    Ok(())
}

fn start(terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
    let mut app = App::default();

    loop {
        terminal.draw(|frame| ui(frame, &app))?;

        if let Ok(Event::Key(key)) = read()
            && key.kind == Press
        {
            match app.current_screen {
                CurrentScreen::BeforeRoom => {
                    match key.code {
                        KeyCode::Char('q') => {
                            return Ok(());
                        }

                        KeyCode::Char(_) => {
                            // draw new cards, and let the user choose the cards
                            app.draw_cards(4);
                            app.current_screen = CurrentScreen::ChooseCard;
                        }

                        _ => {}
                    }
                }

                CurrentScreen::ChooseCard => match key.code {
                    KeyCode::Char(c @ ('1' | '2' | '3' | '4')) => {
                        // safe to use unwrap, and to cast to usize, as we know the input is always 1, 2, 3 or 4
                        let idx = c.to_digit(10).unwrap() as usize;
                        if let Some(card) = app.select_card(idx) {
                            app.handle_card(card);
                        } else {
                            // TODO: display warning that the card at the selected idx has already been used
                        }

                        if app.hand.num_cards_remaining() == 1 {
                            app.has_avoided_room = false;
                            app.current_screen = CurrentScreen::BeforeRoom;
                        }
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
