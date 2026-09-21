use std::{fs::File, io::Write, cmp::{min, max}};

use crate::{editor::error::EditorError, platform::{self, Platform, renderer::{TextureName, color::Color}}};

pub const HEADER: &[u8] = b"MAP"; 

pub const TEXTURES: [TextureName; 3] = [
    TextureName::BackGround, 
    TextureName::MiddleGround,
    TextureName::ForeGround,
];

pub struct Map {
    pub layers: [Layer; 3],
    w: u32,
    h: u32,
}

impl Map {
    pub fn new(w: u32, h: u32) -> Self {
        Self {
            layers: [Layer::new(w, h, 0.5), Layer::new(w, h, 1.0), Layer::new(w, h, 1.5)],
            w,
            h,
        }
    }

    pub fn open_new(path: String) -> Self {
        let mut map = Self::new(0, 0);
        map.open(path);
        map
    }

    pub fn set_tile(&mut self, x: u32, y: u32, t: u8, layer: usize) {
        println!("Setting tile: {t}, on layer: {layer}, at x: {x}, y: {y}");
        self.layers[layer].set_tile(x, y, t);
    }

    pub fn set_tiles(&mut self, t_x: u32, t_y: u32, a_x: u8, a_y: u8, b_x: u8, b_y: u8, layer: usize) {
        let min_x = min(a_x, b_x);
        let min_y = min(a_y, b_y);

        let max_x = max(a_x, b_x);
        let max_y = max(a_y, b_y);

        for x in (min_x..=max_x) {
            for y in (min_y..=max_y) {
                let t = (y * 16) + x;
                self.set_tile(x as u32 + t_x - min_x as u32, y as u32 + t_y - min_y as u32, t, layer);
            }    
        }
    }

    pub fn get_col(&self, x: i32, y: i32) -> Option<&u8> {
        self.layers[1].get_col(x, y)
    }

    pub fn draw(&self, platform: &mut Platform, x: i32, y: i32, c_layer: usize, is_editor: bool, hide_layers: bool) {
        if is_editor {
            let c = match c_layer {
                0 => Color::new(255, 50, 50, 50),
                1 => Color::new(50, 255, 50, 50),
                2 => Color::new(50, 50, 255, 50),
                _ => Color::new(0, 0, 0, 50)
            };
            self.layers[c_layer].draw_outline(platform, x, y, c, c_layer as u32);
        }
        if hide_layers {
            self.layers[c_layer].draw(platform, TEXTURES[c_layer], x, y, is_editor, 3);
        } else {
            for (i, layer) in self.layers.iter().enumerate() {
                layer.draw(platform, TEXTURES[i], x, y, is_editor, i as u32);
            }
        }
        if is_editor {
            platform.renderer.draw(&TextureName::Icons, 16, 0, 16, 16, c_layer as u32, 2, 2, false);
        }
    }

    pub fn save(&mut self, path: String) -> Result<(), EditorError> {
        let mut file = File::create(&path)?;

        let mut buf: Vec<u8> = Vec::new();

        buf.extend_from_slice(HEADER);

        buf.extend_from_slice(&self.w.to_le_bytes());
        buf.extend_from_slice(&self.h.to_le_bytes());

        for layer in &self.layers {
            buf.extend_from_slice(&layer.w.to_le_bytes());
            buf.extend_from_slice(&layer.h.to_le_bytes());

            buf.extend_from_slice(&layer.tiles);
        }

        file.write_all(&buf);

        println!("File saved at: {}", &path);

        Ok(())
    }

    pub fn open(&mut self, path: String) -> Result<(), EditorError> {
        let mut buf = std::fs::read(&path)?;

        if &buf[0..3] != HEADER {
            return Err(EditorError::FileOpen(path));
        }

        let mut offset = 3;

        self.w = u32::from_le_bytes(buf[offset..offset + 4].try_into()?);
        offset += 4;
        self.h = u32::from_le_bytes(buf[offset..offset + 4].try_into()?);
        offset += 4;

        for (i, layer) in &mut self.layers.iter_mut().enumerate() {
            println!("Reading layer: {i}");

            layer.w = u32::from_le_bytes(buf[offset..offset + 4].try_into()?);
            offset += 4;
            layer.h = u32::from_le_bytes(buf[offset..offset + 4].try_into()?);
            offset += 4;

            layer.tiles = buf[offset..offset + (layer.w * layer.h) as usize].to_vec();
            offset += (layer.w * layer.h) as usize;

            println!("Succesfully read layer: {i}");
        }

        println!("Opened file at: {path}");
        
        Ok(())
    }
}

pub struct Layer {
    tiles: Vec<u8>,
    w: u32,
    h: u32,
    scroll: f32,
}

impl Layer {
    pub fn new(w: u32, h: u32, scroll: f32) -> Self {
        Self {
            tiles: vec![0; (w * h) as usize],
            w,
            h,
            scroll,
        }
    }

    pub fn set_tile(&mut self, x: u32, y: u32, t: u8) {
        let id = x * self.h + y;
        if let Some(d) = self.tiles.get_mut(id as usize) {
            *d = t; 
        } else {
            println!("!!    Trying to draw out of map   !!!");
        }
    }

    pub fn get_col(&self, x: i32, y: i32) -> Option<&u8> {
        self.tiles.get((x * self.h as i32 + y) as usize)
    }

    pub fn draw_outline(&self, platform: &mut Platform, x: i32, y: i32, c: Color, d: u32) {
        platform.renderer.draw_rect(
            x as i32,
            y as i32, 
            self.w * 16, self.h * 16, 
            c.r, c.g, c.b, 125, 
            d,
            false);
        platform.renderer.draw_rect(
            x as i32,
            y as i32, 
            self.w * 16,
            self.h * 16, 
            c.r, c.g, c.b, c.a,
            d,
            true);
    }

    pub fn draw(&self, platform: &mut Platform, t: TextureName, x: i32, y: i32, is_editor: bool, depth: u32) {
        for i_x in (0..self.w) {
            for i_y in (0..self.h) {
                let id = self.tiles[(i_x * self.h + i_y) as usize];
                
                let src_x = id as u32 % 16;
                let src_y = id as u32 / 16;

                let x = if is_editor { x } else { (x as f32 * self.scroll) as i32 };
                let y = if is_editor { y } else { (y as f32 * self.scroll) as i32 };

                platform.renderer.draw(&t, 
                    x + (i_x * 16) as i32, 
                    y + (i_y * 16) as i32, 
                    16, 16, 
                    src_x, src_y,
                    depth,
                    false);
            }
        } 
    }
}
