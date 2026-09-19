use std::{cmp::{min, max}, io::{self, Write}, process::Command};

use crate::{editor::{error::EditorError, map::{Map, TEXTURES}, ui::{Ui, UiElement}}, game::Game, platform::{Platform, game::GameInterface, input::Key, renderer::{TextureName, color::Color}}};

pub mod error;
pub mod ui;
pub mod map;

pub const MAP_ROOT: &str = "assets/maps/";
pub const MAP_EXT: &str = ".map";

pub struct Editor {
    ui: Ui,
    map: Map,

    current_tile: u8,
    
    a_t_x: u8,
    a_t_y: u8,
    b_t_x: u8,
    b_t_y: u8,

    current_tool: Tool,
    current_layer: usize,

    //This is just to draw the highlights, drawing pannels provide their own data for local tile
    t_x: i32,
    t_y: i32,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            ui: Ui::new(),
            map: Map::new(18, 16),

            current_layer: 0,
            current_tile: 0,
    
            a_t_x: 0,
            a_t_y: 0,
            b_t_x: 0,
            b_t_y: 0,

            current_tool: Tool::Pen,

            t_x: 0,
            t_y: 0,
        }
    }
}

enum ElementId {
    TogglePen,
    ToggleEraser,
    Save,
    Open,
    New,
    Play,

    Canvas,
    Palette,

    None
}

enum Tool {
    Pen,
    Eraser,
}

impl GameInterface for Editor {
    type Error = EditorError;

    fn init(&mut self, platform: &mut crate::platform::Platform) -> Result<(), Self::Error> {
        platform.renderer.load_texture(TextureName::Ui, "assets/img/ui.png")?;
        platform.renderer.load_texture(TextureName::Icons, "assets/img/icons.png")?;
        platform.renderer.load_texture(TextureName::BackGround, "assets/img/bak_test.png")?;
        platform.renderer.load_texture(TextureName::MiddleGround, "assets/img/mid_test.png")?;
        platform.renderer.load_texture(TextureName::ForeGround, "assets/img/fore_test.png")?;
        platform.renderer.load_texture(TextureName::MenuBack, "assets/img/menu_bg.png")?;

        self.ui.elems.push(UiElement::new(0, 0, 0, 15, 0, 0, ElementId::None as u32, true, false));

        self.ui.elems.push(UiElement::new(0, 0, 0, 0, 4, 0, ElementId::Play as u32, true, true));

        self.ui.elems.push(UiElement::new(0, 32, 0, 0, 3, 0, ElementId::Save as u32, true, true)); 
        self.ui.elems.push(UiElement::new(0, 48, 0, 0, 5, 0, ElementId::Open as u32, true, true)); 
        self.ui.elems.push(UiElement::new(0, 64, 0, 0, 7, 0, ElementId::New as u32, true, true)); 

        self.ui.elems.push(UiElement::new(0, 96, 0, 0, 1, 0, ElementId::TogglePen as u32, true, true));
        self.ui.elems.push(UiElement::new(0, 112, 0, 0, 2, 0, ElementId::ToggleEraser as u32, true, true));
        self.ui.elems.push(UiElement::new(0, 128, 0, 0, 6, 0, ElementId::None as u32, true, true));

        self.ui.elems.push(UiElement::new(320, 0, 0, 15, 0, 0, ElementId::None as u32, true, false));

        self.ui.elems.push(UiElement::new(16, 0, 18, 15, 0, 0, ElementId::Canvas as u32, false, true));
        self.ui.elems.push(UiElement::new(336, 0, 15, 15, 0, 0, ElementId::Palette as u32, false, true));

        Ok(())        
    }

    fn update(&mut self, platform: &mut crate::platform::Platform, delta: f64) -> Result<(), Self::Error> {
        let events = self.ui.tick(platform);

        let (m_x, m_y) = platform.input.get_mouse_pos();

        self.t_x = m_x / 16;
        self.t_y = m_y / 16;

        if platform.input.get_key_down(Key::Up) && self.current_layer != 2 {
            self.current_layer += 1;
            println!("Switching to layer: {}", self.current_layer);
        }
        if platform.input.get_key_down(Key::Down) && self.current_layer != 0 {
            self.current_layer -= 1;
            println!("Switching to layer: {}", self.current_layer);
        }

        for event in events {
            match event {
                ui::UiEvent::Clicked { id, x, y } => {
                    match id {
                        //Side bar buttons
                        //______________
                        val if val == ElementId::TogglePen as u32 => {
                            println!("Toggled pen");
                            self.current_tool = Tool::Pen;
                        }
                        val if val == ElementId::ToggleEraser as u32 => {
                            self.current_tool = Tool::Eraser;
                            println!("Toggled erasee");
                        }
                        val if val == ElementId::Play as u32 => {
                            println!("Trying to play game");

                            Command::new("cargo").args(["run", "--bin", "game"]).spawn()?;
                        }
                        val if val == ElementId::Save as u32 => {
                            print!("Map name: \n] ");
                            io::stdout().flush()?;

                            let mut input = String::new();

                            io::stdin().read_line(&mut input)?;

                            let path = format!("{}{}{}", MAP_ROOT, input.trim(), MAP_EXT);

                            println!("Saving at: \"{path}\"");

                            self.map.save(path)?;
                        }
                        val if val == ElementId::Open as u32 => {
                            print!("Map name\n] ");
                            io::stdout().flush()?;

                            let mut input = String::new();

                            io::stdin().read_line(&mut input)?;

                            let path = format!("{}{}{}", MAP_ROOT, input.trim(), MAP_EXT);

                            println!("Opening file at: \"{path}\"");

                            self.map.open(path)?;
                        }
                        //TODO: Implement that
                        val if val == ElementId::New as u32 => {
                            println!("Cant create new files yet");
                        }

                        //Draw stuff
                        //______________
                        val if val == ElementId::Palette as u32 => {
                            let t_x = x / 16;
                            let t_y = y / 16;

                            self.a_t_x = t_x as u8;
                            self.a_t_y = t_y as u8;

                            println!("Selecting tile: {}", t_x * 16 + t_y);

                            self.current_tile = (t_y * 16 + t_x) as u8;

                            self.current_tool = Tool::Pen;
                        }
                        _ => println!("Unknown element id"),
                    }
                }
                ui::UiEvent::Released { id, x, y } => {
                    match id {
                        val if val == ElementId::Palette as u32 => {
                            println!("Selected tile: t_x: {}, t_y: {}, t_m_x: {}, t_m_y: {}", self.a_t_x, self.a_t_y, self.b_t_x, self.b_t_y);

                        }
                        _ => println!("Unknown element id"),
                    }
                }
                ui::UiEvent::Held { id, x, y } => {
                    match id {
                        val if val == ElementId::Palette as u32 => {
                            self.b_t_x = x as u8 / 16;
                            self.b_t_y = y as u8 / 16;
                        }
                        val if val == ElementId::Canvas as u32 => {
                            let t_x = x / 16;
                            let t_y = y / 16;

                            let (a_x, b_x, a_y, b_y) = match self.current_tool {
                                Tool::Pen => (self.a_t_x, self.b_t_x, self.a_t_y, self.b_t_y),
                                Tool::Eraser => (0, 0, 0, 0),
                            };

                            self.map.set_tiles(t_x as u32, t_y as u32,
                                a_x, a_y, 
                                b_x, b_y, 
                                self.current_layer);
                        }
                        _ => println!("Unknown element id"),
                    }
                }
            }
        }

        Ok(()) 
    }

    fn draw(&mut self, platform: &mut crate::platform::Platform) {
        for x in (0..10) {
            for y in (0..8) {
                platform.renderer.draw(&TextureName::MenuBack, x * 32, y * 32, 32, 32, 1, 0);
            }
        }

        self.map.draw(platform, 16, 0);

        platform.renderer.draw(&TEXTURES[self.current_layer], 336, 0, 256, 256, 0, 0);

        platform.renderer.draw_rect(self.t_x * 16, self.t_y * 16, 16, 16, 205, 207, 229, 50, false);
        platform.renderer.draw_rect(self.t_x * 16, self.t_y * 16, 16, 16, 205, 207, 229, 70, true);

        platform.renderer.set_color(Color::new(255, 255, 255, 255));

        self.ui.draw(platform);
    }
}
