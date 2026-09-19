#![allow(unused)]

use crate::{game::Game, platform::{Platform, error::PlatformError}};

mod platform;
mod game;
mod editor;

fn main() -> Result<(), PlatformError> {
    let mut p = Platform::new()?;
    let mut game = Game::new();
    p.run(game)?;
    Ok(())
}
