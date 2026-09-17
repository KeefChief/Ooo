use std::{thread, time::Duration};

use sdl2::{self, EventPump, render::{Canvas, TextureCreator}, video::{Window, WindowContext}};

use crate::platform::{error::PlatformError, game::GameInterface, renderer::Renderer};

//mods
//____

pub mod renderer;
pub mod game;
pub mod error;

//constants
//_________

const FPS: u64 = 60;
const FRAME_TIME:Duration = Duration::from_nanos(1000000000 / FPS);

//definitions/the rest
//____________________

pub struct Platform {
    events: EventPump,
    pub renderer: Renderer,
}

impl Platform {
    pub fn new() -> Result<Self, PlatformError> {
        let sdl = sdl2::init()?;
        let video = sdl.video()?;

        let window = video
            .window("Ooo", 320, 240)
            .position_centered()
            .build()?;

        let mut canvas = window
            .into_canvas()
            .build()?;

        let image_ctx = sdl2::image::init(sdl2::image::InitFlag::PNG)?;

        let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();

        let renderer = Renderer::new(texture_creator, canvas);

        let events = sdl.event_pump()?;

        Ok(
            Self { 
                events, 
                renderer,
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

            //Event loop
            //__________

            for event in self.events.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => running = 0,
                    _ => {}, 
                }     
            }

            //Game loop
            //_________

            game.update(&mut self, 0.0);

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
            println!("{:?}", elapsed);
        }
        Ok(())
    }
}
