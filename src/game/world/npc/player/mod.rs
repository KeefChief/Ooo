use crate::platform::{Platform, input::Key};
use super::Npc;

impl Npc {
    pub fn t_player(&mut self, platform: &mut Platform) {
        if platform.input.get_key(Key::Right) {
            self.v_x += 1.0;    
        }
        if platform.input.get_key(Key::Left) {
            self.v_x -= 1.0;
        }
        self.v_x *= 0.7;

        self.v_y += 0.4;

        if platform.input.get_key(Key::Jump) {
            if platform.input.get_key_down(Key::Jump) {
                self.v_y = -4.0;
            }
            self.v_y -= 0.23;
        }

        if self.v_x.abs() >= 0.2 {
            self.anim_count += 1;
        } else {
            self.anim_count = 0;
        }

        if self.v_y >= 3.0 {
            self.v_y = 3.0;
        }

        match self.anim_count {
            (0..7) => self.src_x = 0,
            (7..14) => self.src_x = 1,
            (14) => self.src_x = 0,
            _ => self.anim_count = 0
        }
    }
}
