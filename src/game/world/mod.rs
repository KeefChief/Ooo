use crate::game::{utils::flag::Flag, world::npc::{Npc, instantiate}};

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

    pub fn tick(&mut self) {
        for npc in &mut self.npcs {
            npc.tick();
        }
    }
}
