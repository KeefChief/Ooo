#![allow(unused)]

use ooo::{game::Game, platform::{Platform, error::PlatformError}};

fn main() -> Result<(), PlatformError> {
    let mut p = Platform::new(320, 240)?;
    let mut game = Game::new("assets/maps/test.map".to_string());
    p.run(game)?;
    Ok(())
}
