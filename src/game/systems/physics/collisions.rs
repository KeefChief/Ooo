use crate::{editor::map::Map, platform::{Platform, renderer::color::Color}};

#[derive(Clone, Copy, Debug)]
pub struct Collider {
    l: u32,
    r: u32,
    u: u32,
    d: u32,
}

impl Collider {
    pub fn new(l: u32, r: u32, u: u32, d: u32) -> Self {
        Self { l, r, u, d }
    }

    pub fn draw(&self, platform: &mut Platform, x: i32, y: i32) {
        let w = self.r + self.l;
        let h = self.d + self.u;
        // println!("Drawing col at: x: {x}, y: {y}, with width: {w}, and height :{h}");
        platform.renderer.draw_rect(x - self.l as i32, y - self.u as i32, w, h, 50, 255, 50, 125, 1, false);
        platform.renderer.draw_rect(x - self.l as i32, y - self.u as i32, w, h, 50, 255, 50, 125, 1, true);
    }
}

pub trait Collision {
    fn get_col(&self) -> &Collider;

    fn get_pos(&self) -> (f32, f32);

    fn set_x(&mut self, x: f32);

    fn set_y(&mut self, y: f32);

    fn get_vel(&self) -> (f32, f32);

    fn set_vel_y(&mut self, y: f32);

    fn set_vel_x(&mut self, x: f32);

    fn hit_map(&mut self, map: &Map) {
        let col = *self.get_col();
        let (x, y) = self.get_pos();

        let left = (x as i32 - col.l as i32) / 16;
        let right = (x as i32 + col.r as i32) / 16;

        let top = (y as i32 - col.u as i32) / 16;
        let bottom = (y as i32 + col.d as i32) / 16;

        for x in (left..=right) {
            for y in (top..=bottom) {
                if let Some(col) = map.get_col(x, y) {
                    if *col != 0 {
                        self.hit_tile(x, y);
                    }
                }
            }
        }
    }

    fn hit_tile(&mut self, t_x: i32, t_y: i32) {
        let col = *self.get_col();
        let (x, y) = self.get_pos();
        let (v_x, v_y) = self.get_vel();

        let left = x - col.l as f32;
        let right = x + col.r as f32;

        let top = y - col.u as f32;
        let bottom = y + col.d as f32;

        let block_left = (t_x * 16) as f32;
        let block_right = ((t_x + 1) * 16) as f32;

        let block_top = (t_y * 16) as f32;
        let block_bottom = ((t_y + 1) * 16) as f32;

        // println!("{top} > {block_bottom}");
        // println!("{block_top} > {bottom}");

        if right > block_left && left < block_right {
            // println!("In range");
            if bottom > block_top && bottom < block_bottom && v_y > 0.0 {
                // println!("Collidin bottom");
                self.set_y(block_top - col.d as f32);
                self.set_vel_y(0.0);
            } 
            if top < block_bottom && top > block_top && v_y < 0.0 {
                // println!("Collidin top");
                self.set_y(block_bottom + col.u as f32);
                self.set_vel_y(0.0);
            }
        }

        let (x, y) = self.get_pos();
        let (v_x, v_y) = self.get_vel();

        let left = x - col.l as f32;
        let right = x + col.r as f32;

        let top = y - col.u as f32;
        let bottom = y + col.d as f32;

        // println!("Player top: {top}, bottom: {bottom}");
        // println!("Block at: x: {t_x}; y: {t_y}, : {block_top}; {block_bottom}");

        if bottom > block_top && top < block_bottom {
            println!("In range");
            if right > block_left && right < block_right && v_x > 0.0 {
                println!("Collidin right");
                self.set_x(block_left - col.r as f32);
                self.set_vel_x(0.0);
            } if left < block_right && left > block_left && v_x < 0.0 {
                println!("Collidin left");
                self.set_x(block_right + col.l as f32);
                self.set_vel_x(0.0);
            }
        }
    }
}
