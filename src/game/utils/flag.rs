pub struct Flag(pub u32);

impl Flag {
    pub fn set(&mut self, flag: u32) {
        self.0 &= 1 << flag;
    }

    pub fn clear(&mut self, flag: u32) {
        self.0 &= !(1 << flag);
    }

    pub fn switch(&mut self, flag: u32) {
        self.0 ^= 1 << flag;
    }

    pub fn clear_all(&mut self) {
        self.0 = 0;
    }
}
