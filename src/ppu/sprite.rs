#[derive(Clone, Copy)]
pub enum Pallete {
    Zero,
    One,
}

#[derive(Clone, Copy)]
pub struct Sprite {
    pub x: u8,
    pub y: u8,
    pub tile: u8,
    pub overleap_background: bool,
    pub x_flip: bool,
    pub y_flip: bool,
    pub priority: bool,
    pub pallete: Pallete,
}

impl Sprite {
    pub fn new() -> Sprite {
        Sprite {
            x: 0,
            y: 0,
            tile: 0,
            overleap_background: false,
            x_flip: false,
            y_flip: false,
            priority: false,
            pallete: Pallete::Zero,
        }
    }

    //i32 to avoid overflow
    pub fn top_line(&self) -> i32{
        (self.y as i32) - 16
    }

    //32 to avoid overflow
    fn left_column(&self) -> i32 {
        (self.x as i32) - 8
    }

    pub fn set_flags(&mut self, flags: u8) {
        self.overleap_background = flags & 0x80 != 0;
        self.y_flip = flags & 0x40 != 0;
        self.x_flip = flags & 0x20 != 0;
        self.pallete = match flags & 0x10 != 0 {
            false => Pallete::Zero,
            true => Pallete::One,
        }
    }

    pub fn flags(&self) -> u8 {
        let mut register_flags = 0;

        register_flags |= (self.overleap_background as u8) << 7;
        register_flags |= (self.y_flip as u8) << 6;
        register_flags |= (self.x_flip as u8) << 5;

        register_flags |= match self.pallete {
            Pallete::Zero => 0,
            Pallete::One => 1,
        } << 4;

        register_flags
    }
}
