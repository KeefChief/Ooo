use std::{collections::HashMap};

use sdl2::{image::LoadTexture, rect::Rect, render::{Canvas, Texture, TextureCreator}, sys::SDL_Rect, video::{Window, WindowContext}};

use crate::platform::{error::PlatformError};

//Texture handles
//_______________

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum TextureName {
    Player,
}

//Define the renderer itself and its methods
//TODO: Probably add something about the unsafe_texture to not let them roam in RAM eternally
//__________________________

pub struct Renderer {
    textures: HashMap<TextureName, Texture>,
    texture_creator: TextureCreator<WindowContext>,
    pub canvas: Canvas<Window>
}

impl Renderer {
    pub fn new(texture_creator: TextureCreator<WindowContext>, canvas: Canvas<Window>) -> Self {
        Self { 
            textures: HashMap::new(),
            texture_creator,
            canvas,
        }
    }

    //Load a texture from a string path
    //TODO: Add a system to only have to use the file name and specify a root path in platform too
    //maybe; Do the same thing for stuff like maps later
    //_________________________

    pub fn load_texture(&mut self, t: TextureName, f: &str) -> Result<(), PlatformError> {
        let texture = self.texture_creator.load_texture(f)?;

        self.textures.insert(t, texture);

        Ok(())
    }

    //Draw stuff on screen, x and y as dst coords, same w and h for the dst and src, src picks using
    //src_x * w (same for y) so pick a tile using 'atlas' coords
    //__________________________

    pub fn draw(&mut self, t: TextureName, x:i32, y:i32, w:u32, h:u32, src_x:i32, src_y:i32) 
    -> Result<(), PlatformError>{
        let texture = self.textures.get(&t);

        let dst = Rect::new(x, y, w, h);

        let src = Rect::new(src_x * w as i32, src_y * h as i32, w, h);

        if let Some(texture) = texture {
            self.canvas.copy(texture, src, dst);
        }
        Ok(())
    }
}
