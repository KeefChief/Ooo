use crate::game::utils::flag::Flag;
use crate::game::world::World;
use crate::game::world::npc::instantiate;
use crate::platform::error::PlatformError;
use crate::platform::{Platform, game::GameInterface};
use crate::game::error::GameError;
use crate::platform::renderer::TextureName;

mod error;
mod world;
mod systems;
pub mod utils;

pub struct Game {
    world: World,
    player: usize,
    x: i32,
    y: i32,
}

impl Game {
    pub fn new(path: String) -> Self {
        Self {
            world: World::new(path),
            player: 0,
            x: 0,
            y: 0,
        }
    }
}

//See platform/game for more info about this trait
//_______________

impl GameInterface for Game {
    type Error = GameError;

    //Executed only once at launch
    //_______________

    fn init(&mut self, platform: &mut Platform) -> Result<(), Self::Error> {
        platform.renderer.load_texture(TextureName::Player, "assets/img/player.png")?; 
        platform.renderer.load_texture(TextureName::BackGround, "assets/img/bak_test.png")?;
        platform.renderer.load_texture(TextureName::MiddleGround, "assets/img/mid_test.png")?;
        platform.renderer.load_texture(TextureName::ForeGround, "assets/img/fore_test.png")?;

        self.player = instantiate(&mut self.world, 0, 0.0, 90.0, 16, 16, Flag(0));

        Ok(())
    }

    //Update, 60 times per frame
    //TODO: Maybe add delta time if i find out I need it(shouldn't need it though since the game is
    //small)
    //________________

    fn update(&mut self, platform: &mut Platform, delta: f64) -> Result<(), Self::Error> {
        self.world.tick(platform);

        self.x = self.world.npcs.get(self.player).unwrap().x as i32;

        Ok(())
    }

    //Draw, 60 per frame too, happens right after update
    //________________

    fn draw(&mut self, platform: &mut Platform) {
        self.world.draw(platform, self.x, self.y);
    }
}
