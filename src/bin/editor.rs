#![allow(unused)]

use ooo::game::Game;
use ooo::platform::{Platform, error::PlatformError};
use ooo::editor::Editor;

fn main() -> Result<(), PlatformError> {
    let platform = Platform::new(592, 256)?; 
    let editor = Editor::new();
    platform.run(editor)?;
    Ok(())
}
