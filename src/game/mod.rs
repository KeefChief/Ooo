use crate::game::utils::flag::Flag;
use crate::game::world::World;
use crate::game::world::npc::instantiate;
use crate::platform::error::PlatformError;
use crate::platform::{Platform, game::GameInterface};
use crate::game::error::GameError;
use crate::platform::renderer::TextureName;

mod error;
mod world;
pub mod utils;

pub struct Game {
    world: World,
}

impl Game {
    pub fn new() -> Self {
        Self {
            world: World::new(),
        }
    }
}

impl GameInterface for Game {
    type Error = GameError;

    fn init(&mut self, platform: &mut Platform) -> Result<(), Self::Error> {
        platform.renderer.load_texture(TextureName::Player, "assets/img/player.png")?; 

        instantiate(&mut self.world, 0, 0, 1, Flag(0));
        instantiate(&mut self.world, 0, 0, 21, Flag(0));

        Ok(())
    }

    fn update(&mut self, platform: &mut Platform, delta: f64) {
        self.world.tick();
    }

    fn draw(&mut self, platform: &mut Platform) {
        platform.renderer.draw(TextureName::Player, 0, 0, 16, 16, 0, 0);
        platform.renderer.draw(TextureName::Player, 304, 224, 16, 16, 1, 0);
    }
}
