use crate::platform::{Platform, input::Key};
use super::Npc;

impl Npc {
    pub fn t_player(&mut self, platform: &mut Platform) {
        if platform.input.get_key(Key::Right) {
            self.v_x += 0.8;    
        }
        if platform.input.get_key(Key::Left) {
            self.v_x -= 0.8;
        }
        self.v_x *= 0.7;

        self.v_y += 0.2;

        if platform.input.get_key(Key::Jump) {
            if platform.input.get_key_down(Key::Jump) {
                self.v_y = -3.0;
            }
            self.v_y -= 0.1;
        }

        if self.y >= 100.0 && self.v_y >= 0.0 {
            self.v_y = 0.0;
            self.y = 100.0;
        }
    }
}
