use std::{collections::HashMap};

use sdl2::{image::LoadTexture, pixels::Color, rect::Rect, render::{Canvas, Texture, TextureCreator}, sys::SDL_Rect, video::{Window, WindowContext}};

use crate::platform::{Platform, error::PlatformError};

pub mod color;

//Texture handles
//_______________

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub enum TextureName {
    Player,
    Npcs,

    Ui,
    Icons,

    BackGround,
    MiddleGround,
    ForeGround,
    
    MenuBack,
}

pub struct DrawCommand {
    d: Drawable,
    depth: u32,
}

enum Drawable {
    Img {
        t: TextureName,
        dst: Rect,
        src: Rect,
    },
    Rect {
        rect: Rect,
        c: Color,
        fill: bool,
    }
}

impl DrawCommand {
    pub fn new(depth: u32, d: Drawable) -> Self {
        Self { depth, d }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, textures: &HashMap<TextureName, Texture>) {
        match self.d {
            Drawable::Img { t, dst, src } => {
                let t = textures.get(&t);
                if let Some(t) = t {
                    canvas.copy(t, src, dst);
                }
            }
            Drawable::Rect { rect, c, fill } => {
                canvas.set_draw_color(c);
                if fill {
                    canvas.fill_rect(rect);
                } else {
                    canvas.draw_rect(rect);
                }
                canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));
            }
        }
    }
}

//Define the renderer itself and its methods
//TODO: Probably add something about the unsafe_texture to not let them roam in RAM eternally
//__________________________

pub struct Renderer {
    textures: HashMap<TextureName, Texture>,
    texture_creator: TextureCreator<WindowContext>,
    commands: Vec<DrawCommand>,
    pub canvas: Canvas<Window>
}

impl Renderer {
    pub fn new(texture_creator: TextureCreator<WindowContext>, canvas: Canvas<Window>) -> Self {
        Self { 
            textures: HashMap::new(),
            texture_creator,
            canvas,
            commands: Vec::new(),
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

    pub fn draw(&mut self, t: &TextureName, mut x:i32, mut y:i32, w:u32, h:u32, src_x:u32, src_y:u32, depth: u32, centered: bool) 
    -> Result<(), PlatformError>{

        if centered {
            x -= (w / 2) as i32;
            y -= (h / 2) as i32;
        }

        let dst = Rect::new(x, y, w, h);

        let src = Rect::new((src_x * w) as i32, (src_y * h) as i32, w, h);

        let d = Drawable::Img { t: *t, dst, src };
        let command = DrawCommand::new(depth, d);
        self.commands.push(command);

        Ok(())
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, w: u32, h: u32, r: u8, g: u8, b: u8, a: u8, depth: u32, fill: bool) 
    {
        let rect = Rect::new(x, y, w, h);
        let d = Drawable::Rect { rect, c: Color::RGBA(r, g, b, a), fill };
        self.commands.push(DrawCommand::new(depth, d));
    }

    pub fn set_color(&mut self, c: color::Color) {
        self.canvas.set_draw_color(Color::RGBA(c.r, c.g, c.b, c.a));
    }

    pub fn present(&mut self) {
        self.commands.sort_by_key(|cmd| cmd.depth);
        for command in &self.commands {
            command.draw(&mut self.canvas, &self.textures);
        }
        self.commands.clear();
    }
}
