use crate::{editor::map::Map, game::{systems::physics::collisions::{Collider, Collision}, utils::flag::Flag, world::World}, platform::{Platform, renderer::TextureName}};

mod player;

#[derive(Debug)]
pub struct Npc {
    kind: u32,

    pub x: f32,
    pub y: f32,
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

    col : Collider,
}

pub fn instantiate(world: &mut World, kind: u32, x: f32, y: f32, w: u32, h: u32, flags: Flag) -> usize {
    let id = world.npcs.len();
    world.npcs.push(Npc::new(kind, x, y, w, h, flags)); 
    id
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
            flags,
            col: Collider::new(2, 2, 2, 8)
        }
    }

    //Self explanatory
    //_______________

    pub fn tick(&mut self, platform: &mut Platform, map: &Map) {
        match self.kind {
            0 => self.t_player(platform),
            _ => println!("Unknown entity kind: {}", self.kind)
        }
        
        self.hit_map(map);

        self.x += self.v_x;
        self.y += self.v_y;
    }

    //Same over here I think
    //_______________

    pub fn draw(&self, platform: &mut Platform, x: i32, y: i32) {
        let texture = if self.is_player == false {
            TextureName::Npcs
        } else {
            TextureName::Player
        };

        platform.renderer.draw(&texture, self.x as i32 - x, self.y as i32 - y, self.w, self.h, self.src_x, self.src_y, 1, true);
        self.col.draw(platform, self.x as i32, self.y as i32);
    }
}

impl Collision for Npc {
    fn get_col(&self) -> &Collider {
        &self.col
    }

    fn get_pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn get_vel(&self) -> (f32, f32) {
        (self.v_x, self.v_y)
    }

    fn set_x(&mut self, x: f32) {
        self.x = x
    }

    fn set_y(&mut self, y: f32) {
        self.y = y
    }

    fn set_vel_y(&mut self, y: f32) {
        self.v_y = y
    }

    fn set_vel_x(&mut self, x: f32) {
        self.v_x = x
    }
}
