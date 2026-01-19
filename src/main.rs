//! Play Scoundrel on your CLI.

extern crate core;

use crate::game::run_game::run_game;

mod app;
mod cards;
mod game;
mod ui;

fn main() -> color_eyre::Result<()> {
    ratatui::run(run_game)?;
    Ok(())
}