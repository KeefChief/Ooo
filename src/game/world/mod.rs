use crate::{editor::map::Map, game::{utils::flag::Flag, world::npc::{Npc, instantiate}}, platform::Platform};

pub mod npc;

pub struct World {
    npcs: Vec<Npc>,
    map: Map,
}

impl World {
    pub fn new(path: String) -> Self {
        Self {
            npcs: Vec::new(),
            map: Map::open_new(path)
        }
    }

    pub fn tick(&mut self, platform: &mut Platform) {
        for npc in &mut self.npcs {
            npc.tick(platform);
        }
    }

    pub fn draw(&self, platform: &mut Platform) {
        for npc in &self.npcs {
            npc.draw(platform);
        }
        self.map.draw(platform, 0, 0);
    }
}
