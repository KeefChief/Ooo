use std::collections::{HashMap, hash_map};
use sdl2::keyboard::Keycode;

#[derive(Debug, Clone, Copy)]
pub enum Key {
    Left,
    Right,
    Up,
    Down,
    Jump,
    Count,
}

pub struct Input {
    keys_down: [bool; Key::Count as usize],
    keys_up: [bool; Key::Count as usize],
    keys: [bool; Key::Count as usize],
    map: HashMap<Keycode, Key>
}

impl Input {
    pub fn new() -> Self {
        let map = HashMap::from([
            (Keycode::Q, Key::Left),
            (Keycode::D, Key::Right),
            (Keycode::Z, Key::Up),
            (Keycode::S, Key::Down),
            (Keycode::Space, Key::Jump),
        ]);

        Self {
            keys_down: [false; Key::Count as usize],
            keys_up: [false; Key::Count as usize],
            keys: [false; Key::Count as usize],
            map,
        }
    }

    pub fn get_key_down(&self, key: Key) -> bool {
        self.keys_down[key as usize]
    }

    pub fn get_key_up(&self, key: Key) -> bool {
        self.keys_up[key as usize]
    }

    pub fn get_key(&self, key: Key) -> bool {
        self.keys[key as usize]
    }

    pub fn press_key(&mut self, keycode: Keycode) {
        let key = self.map.get(&keycode);

        if let Some(key) = key {
            let key = *key;
            self.keys_down[key as usize] = true;
            self.keys_up[key as usize] = false;
            self.keys[key as usize] = true;
        }
    }

    pub fn release_key(&mut self, keycode: Keycode) {
        let key = self.map.get(&keycode);

        if let Some(key) = key {
            let key = *key;
            self.keys_down[key as usize] = false;
            self.keys_up[key as usize] = true;
            self.keys[key as usize] = false;
        }
    }

    pub fn reset_down(&mut self) {
        self.keys_down.fill(false);
    }
}
