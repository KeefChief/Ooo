use crate::{game::{utils::flag::Flag, world::World}, platform::{Platform, renderer::TextureName}};

mod player;

#[derive(Debug)]
pub struct Npc {
    kind: u32,

    x: f32,
    y: f32,
    v_x: f32,
    v_y: f32,

    src_x: u32,
    src_y: u32,

    w: u32,
    h: u32,

    anim_count: i32,
    action_count: i32,

    is_player: bool,

    flags: Flag,
}

pub fn instantiate(world: &mut World, kind: u32, x: f32, y: f32, w: u32, h: u32, flags: Flag) {
    world.npcs.push(Npc::new(kind, x, y, w, h, flags)); 
}

impl Npc {

    //May need to add more parameters later/ a way to specify special paramters (like by giving a
    //NPC with the special paramters you want and then this assigns them to it, and assigns the rest
    //like normal)
    pub fn new(kind: u32, x: f32, y: f32, w: u32, h: u32, flags: Flag) -> Self {
        Self { 
            kind, 
            x, 
            y, 
            v_x: 0.0, 
            v_y: 0.0, 
            src_x: 0,
            src_y: 0, 
            w, 
            h, 
            anim_count: 0, 
            action_count: 0,
            is_player: if kind == 0 { true } else { false },
            flags
        }
    }

    //Self explanatory
    //_______________

    pub fn tick(&mut self, platform: &mut Platform) {
        self.x += self.v_x;
        self.y += self.v_y;

        match self.kind {
            0 => self.t_player(platform),
            _ => println!("Unknown entity kind: {}", self.kind)
        }
    }

    //Same over here I think
    //_______________

    pub fn draw(&self, platform: &mut Platform) {
        let texture = if self.is_player == false {
            TextureName::Npcs
        } else {
            TextureName::Player
        };

        platform.renderer.draw(texture, self.x as i32, self.y as i32, self.w, self.h, self.src_x, self.src_y);
    }
}
