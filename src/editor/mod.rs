use crate::{editor::{ElementId::{ToggleEraser, TogglePen}, error::EditorError, ui::{Ui, UiElement}}, platform::{game::GameInterface, renderer::TextureName}};

pub mod error;
pub mod ui;

pub struct Editor {
    ui: Ui,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            ui: Ui::new(),
        }
    }
}

enum ElementId {
    TogglePen,
    ToggleEraser,
}

impl GameInterface for Editor {
    type Error = EditorError;

    fn init(&mut self, platform: &mut crate::platform::Platform) -> Result<(), Self::Error> {
        platform.renderer.load_texture(TextureName::Ui, "assets/img/ui.png")?;
        platform.renderer.load_texture(TextureName::Icons, "assets/img/icons.png")?;

        self.ui.elems.push(UiElement::new(0, 0, 0, 0, 1, 0, TogglePen as u32));
        self.ui.elems.push(UiElement::new(16, 0, 0, 0, 2, 0, ToggleEraser as u32));

        Ok(())        
    }

    fn update(&mut self, platform: &mut crate::platform::Platform, delta: f64) -> Result<(), Self::Error> {
        self.ui.tick(platform);

        Ok(()) 
    }

    fn draw(&mut self, platform: &mut crate::platform::Platform) {
        self.ui.draw(platform);
    }
}
