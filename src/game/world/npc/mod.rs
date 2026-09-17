use crate::{game::{utils::flag::Flag, world::World}, platform::{Platform, renderer::TextureName}};

pub struct Npc {
    kind: u32,

    x: i32,
    y: i32,
    v_x: i32,
    v_y: i32,

    src_x: u32,
    src_y: u32,

    w: u32,
    h: u32,

    anim_count: i32,
    action_count: i32,

    is_player: bool,

    flags: Flag,
}

pub fn instantiate(world: &mut World, x: i32, y: i32, kind: u32, flags: Flag) {
    world.npcs.push(Npc::new(x, y, kind, flags)); 
}

impl Npc {
    pub fn new(x: i32, y: i32, kind: u32, flags: Flag) -> Self {
        Self { 
            kind, 
            x, 
            y, 
            v_x: 0, 
            v_y: 0, 
            src_x: 0,
            src_y: 0, 
            w: 0, 
            h: 0, 
            anim_count: 0, 
            action_count: 0,
            is_player: false,
            flags
        }
    }

    pub fn tick(&mut self) {
        println!("I am NPC of kind: {}", self.kind);
    }

    pub fn draw(&self, platform: &mut Platform) {
        let texture = if self.is_player == false {
            TextureName::Npcs
        } else {
            TextureName::Player
        };

        platform.renderer.draw(texture, self.x, self.y, self.w, self.h, self.src_x, self.src_y);
    }
}
