use crate::platform::error::PlatformError;
use crate::platform::{Platform, game::GameInterface};
use crate::game::error::GameError;
use crate::platform::renderer::TextureName;

mod error;

pub struct Game {

}

impl Game {
    pub fn new() -> Self {
        Self {}
    }
}

impl GameInterface for Game {
    type Error = GameError;

    fn init(&mut self, platform: &mut Platform) -> Result<(), Self::Error> {
        platform.renderer.load_texture(TextureName::Player, "assets/img/player.png")?; 

        Ok(())
    }

    fn update(&mut self, platform: &mut Platform, delta: f64) {
        println!("HEya");
    }

    fn draw(&mut self, platform: &mut Platform) {
        println!("Im drawin");
        platform.renderer.draw(TextureName::Player, 0, 0, 16, 16, 0, 0);
        platform.renderer.draw(TextureName::Player, 304, 224, 16, 16, 1, 0);
    }
}
