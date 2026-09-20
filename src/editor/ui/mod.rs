use crate::platform::{Platform, input::Key, renderer::TextureName};

pub enum UiEvent {
    Clicked{
        id: u32,
        x: i32,
        y: i32,
    },
    Released{
        id: u32,
        x: i32,
        y: i32,
    },
    Held {
        id: u32,
        x: i32,
        y: i32,
    }
}

pub struct Ui {
    pub elems: Vec<UiElement>,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            elems: Vec::new(),
        }
    }

    pub fn tick(&mut self, platform: &mut Platform) -> Vec<UiEvent> {
        let mut events: Vec<UiEvent> = Vec::new();

        for elem in &mut self.elems {
            elem.tick(platform, &mut events);
        }

        events
    }

    pub fn draw(&self, platform: &mut Platform) {
        for elem in &self.elems {
            elem.draw(platform);
        }
    }
}

pub struct UiElement {
    x: i32,
    y: i32,

    w: u32,
    h: u32,

    //For icons (0, 0 is an empty icon)
    i_src_x: u32,
    i_src_y: u32,

    is_clicked: bool,

    state: u32,

    id: u32,

    show: bool,

    is_button: bool,
}

impl UiElement {
    pub fn new(x: i32, y: i32, w: u32, h: u32, i_src_x: u32, i_src_y: u32, id: u32, show: bool, is_button: bool) -> Self {
        Self {
            x,
            y,

            //w and h start at 0 (a w: 0, h: 0 button is a one block button)
            w,
            h,

            i_src_x,
            i_src_y,

            is_clicked: false,

            state: 0,

            id,

            show,

            is_button,
        }
    }

    pub fn tick(&mut self, platform: &Platform, events: &mut Vec<UiEvent>) {
        if self.is_button == false {
            return
        }

        let (m_x, m_y) = platform.input.get_mouse_pos();

        let left = self.x;
        let right = left + (self.w + 1) as i32 * 16;

        let top = self.y;
        let bottom = top + (self.h + 1) as i32 * 16;

        self.state = 0;

        let x = m_x - self.x;
        let y = m_y - self.y;

        if m_x >= left && m_x < right && m_y >= top && m_y < bottom {
            self.state = if platform.input.get_key(Key::MouseLeft) { 
                events.push(UiEvent::Held { id: self.id, x, y });
                if platform.input.get_key_down(Key::MouseLeft) {
                    events.push(UiEvent::Clicked { id: self.id, x, y });
                }
                2 
            } 
            else { 
                1 
            };

            if platform.input.get_key_up(Key::MouseLeft) {
                events.push(UiEvent::Released { id: self.id, x, y });
            }
        }
    }

    pub fn draw(&self, platform: &mut Platform) {
        if self.show == false {
            return
        }

        let mut src_x = if self.w == 0 { 3 } else { 0 };
        let mut base_src_y = if self.h == 0 { 3 } else { 0 } + self.state * 4;
        let mut src_y = base_src_y;

        for x in (0..=self.w) {
            if x == 1 || (x == self.w && self.w != 0) { src_x += 1 }
            for y in (0..=self.h) {
                if y == 1 || (y == self.h && self.h != 0) { src_y += 1 }
                platform.renderer.draw(&TextureName::Ui, 
                    self.x + x as i32 * 16,
                    self.y + y as i32 * 16,
                    16, 16,
                    src_x, src_y,
                    1,
                    false);
            }
            src_y = base_src_y;
        }

        platform.renderer.draw(&TextureName::Icons, self.x, self.y, 16, 16, self.i_src_x, self.i_src_y, 1, false);
    }
}
