//! Play Scoundrel on your CLI.

extern crate core;

use crate::app::{App, CurrentScreen, HAND_SIZE};
use crate::ui::ui;
use crossterm::event::{Event, KeyCode, KeyEventKind, read};
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

        match app.current_screen {
            CurrentScreen::Welcome => {
                if let Event::Key(key) = read()?
                    && key.kind == KeyEventKind::Press
                {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        _ => {
                            app.current_screen = CurrentScreen::BeforeRoom;
                            app.draw_cards(HAND_SIZE);
                        }
                    }
                }
            }

            CurrentScreen::BeforeRoom => {
                if app.has_avoided_room {
                    app.current_screen = CurrentScreen::ChooseCard;
                    continue;
                }

                if let Event::Key(key) = read()?
                    && key.kind == KeyEventKind::Press
                {
                    match key.code {
                        // user chooses to enter room
                        KeyCode::Char('y') => {
                            app.current_screen = CurrentScreen::ChooseCard;
                        }

                        // user avoids room
                        KeyCode::Char('n') => {
                            // draw new cards
                            app.put_back_cards();
                            app.draw_cards(HAND_SIZE);
                            app.has_avoided_room = true;

                            app.current_screen = CurrentScreen::BeforeRoom;
                        }

                        KeyCode::Char('q') => {
                            return Ok(());
                        }

                        _ => {}
                    }
                }
            }

            CurrentScreen::ChooseCard => {
                if let Event::Key(key) = read()?
                    && key.kind == KeyEventKind::Press
                {
                    match key.code {
                        KeyCode::Char(c @ ('1' | '2' | '3' | '4')) => {
                            // safe to use unwrap, and to cast to usize, as we know the input is always 1, 2, 3 or 4
                            let idx = c.to_digit(10).unwrap() as usize;
                            if let Some(card) = app.select_card(idx) {
                                match app.handle_card(card) {
                                    Ok(Some(next_screen)) => app.current_screen = next_screen,
                                    Err(error) => {
                                        app.notifications.push(error.to_string());
                                    }
                                    _ => {}
                                }
                            } else {
                                app.notifications
                                    .push("Card has already been used!".to_string());
                            }
                        }
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        _ => {}
                    }
                }

                // TODO: refactor this: this is not good design, as we should not have to care about what the current screen is
                if app.current_screen != CurrentScreen::ChooseWeaponOrBareKnuckle && app.current_screen != CurrentScreen::Lost {
                    draw_cards_or_end_game(&mut app);
                }
            }

            CurrentScreen::ChooseWeaponOrBareKnuckle => {
                if let Event::Key(key) = read()?
                    && key.kind == KeyEventKind::Press
                {
                    let next_screen = match key.code {
                        KeyCode::Char('y') => app.fight_creature_with_weapon(),

                        KeyCode::Char('n') => app.fight_creature_bare_knuckle(),

                        _ => Ok(None),
                    };

                    match next_screen {
                        Ok(Some(next_screen)) => {
                            app.current_screen = next_screen;
                        }
                        Err(error) => {
                            app.notifications.push(error.to_string());
                        }
                        _ => {
                            // default transition
                            draw_cards_or_end_game(&mut app);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn draw_cards_or_end_game(app: &mut App) {
    if app.hand.num_cards_remaining() == 1 {
        app.has_avoided_room = false;
        let enough_cards_left_in_deck = app.draw_cards(HAND_SIZE);
        if !enough_cards_left_in_deck {
            // end the game if there are not enough cards remaining to play the game
            app.current_screen = CurrentScreen::Won;
        } else {
            app.current_screen = CurrentScreen::BeforeRoom;
        }
    } else {
        app.current_screen = CurrentScreen::ChooseCard;
    }
}