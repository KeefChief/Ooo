use crate::{game::{utils::flag::Flag, world::npc::{Npc, instantiate}}, platform::Platform};

pub mod npc;

pub struct World {
    npcs: Vec<Npc>,
}

impl World {
    pub fn new() -> Self {
        Self {
            npcs: Vec::new(),
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
    }
}
