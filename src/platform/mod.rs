use std::{sync::mpsc::{self, Receiver}, thread, time::Duration};

use sdl2::{self, EventPump, pixels::Color, render::{Canvas, TextureCreator}, video::{Window, WindowContext}};

use crate::platform::{error::PlatformError, game::GameInterface, input::Input, renderer::Renderer};

//mods
//____

pub mod renderer;
pub mod game;
pub mod error;
pub mod input;

//constants
//_________

const FPS: u64 = 60;
const FRAME_TIME:Duration = Duration::from_nanos(1000000000 / FPS);

//definitions/the rest
//____________________

pub struct Platform {
    events: EventPump,
    pub renderer: Renderer,
    pub input: Input,
}

impl Platform {
    pub fn new(w: u32, h: u32) -> Result<Self, PlatformError> {
        let sdl = sdl2::init()?;
        let video = sdl.video()?;

        let window = video
            .window("Ooo", w * 2, h * 2)
            .position(0, 0)
            .build()?;

        let mut canvas = window
            .into_canvas()
            .build()?;

        canvas.set_logical_size(w, h);
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

        let image_ctx = sdl2::image::init(sdl2::image::InitFlag::PNG)?;

        let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();

        let renderer = Renderer::new(texture_creator, canvas);

        let events = sdl.event_pump()?;

        let input = Input::new();

        Ok(
            Self { 
                events, 
                renderer,
                input,
            }
        )
    }

    //Whole platform and game loop
    //____________________________

    pub fn run<G: GameInterface>(mut self, mut game: G) -> Result<(), PlatformError> {
        game.init(&mut self);

        let mut running = 1;
        
        while running != 0 {
            let start = std::time::Instant::now();

            self.input.reset_down();

            //Event loop
            //__________

            for event in self.events.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => running = 0,
                    sdl2::event::Event::KeyDown { keycode: Some(keycode), repeat: false, .. } => {
                        self.input.press_key(input::Type::Key(keycode));
                    }
                    sdl2::event::Event::KeyUp { keycode: Some(keycode), repeat: false, ..} => {
                        self.input.release_key(input::Type::Key(keycode));
                    }
                    sdl2::event::Event::MouseMotion { x, y, ..} => {
                        self.input.update_mouse_pos(x, y);
                    }
                    sdl2::event::Event::MouseButtonDown { mouse_btn, .. } => {
                        self.input.press_key(input::Type::Mouse(mouse_btn));
                    }
                    sdl2::event::Event::MouseButtonUp { mouse_btn, .. } => {
                        self.input.release_key(input::Type::Mouse(mouse_btn));
                    }
                    _ => {}, 
                }     
            }

            //Game loop
            //_________

            game.update(&mut self, 0.0);

            self.renderer.canvas.set_draw_color(Color::RGB(37, 50, 63));

            self.renderer.canvas.clear();

            game.draw(&mut self);

            self.renderer.canvas.present();
            
            //Dont add stuff after that line
            //frame time processing stuff
            //______________________________
            
            let elapsed = start.elapsed();

            if elapsed < FRAME_TIME {
                thread::sleep(FRAME_TIME - elapsed);
            }

            let elapsed = start.elapsed();
            // println!("{:?}", elapsed);
        }
        Ok(())
    }
}
