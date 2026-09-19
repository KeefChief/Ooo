use std::collections::{HashMap, hash_map};
use sdl2::{keyboard::Keycode, mouse::MouseButton};

#[derive(Debug, Clone, Copy)]
pub enum Key {
    Left,
    Right,
    Up,
    Down,
    Jump,
    MouseLeft,
    Count,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type {
    Key(Keycode),
    Mouse(MouseButton),
}

pub struct Input {
    keys_down: [bool; Key::Count as usize],
    keys_up: [bool; Key::Count as usize],
    keys: [bool; Key::Count as usize],
    map: HashMap<Type, Key>,

    m_x: i32,
    m_y: i32,
}

impl Input {
    pub fn new() -> Self {
        let map = HashMap::from([
            (Type::Key(Keycode::Q), Key::Left),
            (Type::Key(Keycode::D), Key::Right),
            (Type::Key(Keycode::Z), Key::Up),
            (Type::Key(Keycode::S), Key::Down),
            (Type::Key(Keycode::Space), Key::Jump),
            (Type::Mouse(MouseButton::Left), Key::MouseLeft),
        ]);

        Self {
            keys_down: [false; Key::Count as usize],
            keys_up: [false; Key::Count as usize],
            keys: [false; Key::Count as usize],
            map,

            m_x: 0,
            m_y: 0,
        }
    }

    //Keyboard stuff here
    //_______________________________

    //Access the different key lists
    //____________

    pub fn get_key_down(&self, key: Key) -> bool {
        self.keys_down[key as usize]
    }

    pub fn get_key_up(&self, key: Key) -> bool {
        self.keys_up[key as usize]
    }

    pub fn get_key(&self, key: Key) -> bool {
        self.keys[key as usize]
    }

    //press a key to update the key lists accessed by get_key*_up/_down* functions
    //____________

    pub fn press_key(&mut self, key: Type) {
        let key = self.map.get(&key);

        if let Some(key) = key {
            let key = *key;
            self.keys_down[key as usize] = true;
            self.keys_up[key as usize] = false;
            self.keys[key as usize] = true;
        }
    }

    //release a key to update the key lists accessed by get_key*_up/_down* functions
    //____________

    pub fn release_key(&mut self, key: Type) {
        let key = self.map.get(&key);

        if let Some(key) = key {
            let key = *key;
            self.keys_down[key as usize] = false;
            self.keys_up[key as usize] = true;
            self.keys[key as usize] = false;
        }
    }
    
    //Reset the keys down to avoid stuck keys
    //____________

    pub fn reset_down(&mut self) {
        self.keys_down.fill(false);
    }

    //Mouse stuff down here 
    //_______________________________
    
    pub fn get_mouse_pos(&self) -> (i32, i32) {
        (self.m_x, self.m_y)
    }

    //Update the current mouse position
    //____________

    pub fn update_mouse_pos(&mut self, x: i32, y: i32) {
        (self.m_x, self.m_y) = (x, y);
    }
}
