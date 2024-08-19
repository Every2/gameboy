#[derive(Clone, Copy)]
enum TilePixelValue {
    Zero,
    One,
    Two,
    Three,
}

type Tile = [[TilePixelValue; 8]; 8];

fn empty_tile() -> Tile {
    [[TilePixelValue::Zero; 8]; 8]
}

#[derive(Clone, Copy)]
enum Pallete {
    Zero,
    One,
}

#[derive(Eq, PartialEq)]
enum Interrupts {
    None,
    VBlank,
    LCDStat,
    Both,
}

impl Interrupts {
    fn add(&mut self, other: Interrupts) {
        match self {
            Interrupts::None => *self = other,
            Interrupts::VBlank if other == Interrupts::LCDStat => *self = Interrupts::Both,
            Interrupts::LCDStat if other == Interrupts::VBlank => *self = Interrupts::Both,
            _ => {}
        };
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Hblank,
    Vblank,
    OAMAccess,
    VRAMAccess,
}

impl std::convert::From<Mode> for u8 {
    fn from(value: Mode) -> Self {
        match value {
            Mode::Hblank => 0,
            Mode::Vblank => 1,
            Mode::OAMAccess => 2,
            Mode::VRAMAccess => 3,
        }
    }
}


#[derive(Clone, Copy)]
enum TileMap {
    //0x9800 - 0x9bff
    Low,
    //0x9c00 - 0x9fff
    High,
}

impl TileMap {
    fn base(self) -> u16 {
        match self {
            TileMap::Low => 0x1800,
            TileMap::High => 0x1c00,
        }
    }
}

#[derive(Clone, Copy)]
struct Sprite {
    x: u8,
    y: u8,
    tile: u8,
    overleap_background: bool,
    x_flip: bool,
    y_flip: bool,
    priority: bool,
    pallete: Pallete,
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

    //i8 to avoid underflow
    fn top_line(&self) -> i8 {
        (self.y as i8) - 16
    }

    //i8 to avoid underflow
    fn left_column(&self) -> i8 {
        (self.x as i8) - 8
    }

    fn set_flags(&mut self, flags: u8) {
        self.overleap_background = flags & 0x80 != 0;
        self.y_flip = flags & 0x40 != 0;
        self.x_flip = flags & 0x20 != 0;
        self.pallete = match flags & 0x10 != 0 {
            false => Pallete::Zero,
            true => Pallete::One,
        }
    }

    fn flags(&self) -> u8 {
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

struct Ppu {
    vram: [u8; 0x2000],
    tile_set: [Tile; 384],
    //Gameboy ppu can display 40 objects (sprites)
    oam: [Sprite; 40],
    cycles: u16,
    lcd_enabled: bool,
    mode: Mode,
    line: u8,
    vb_interrupt_enabled: bool,
    oam_is_acessed: bool,
    line_check: u8,
    lck_interrupt_enabled: bool,
    line_equals_line_check: bool,
    hb_interrupt_enabled: bool,
}

impl Ppu {
    fn new() -> Ppu {
        Ppu {
            vram: [0; 0x2000],
            tile_set: [empty_tile(); 384],
            oam: [Sprite::new(); 40],
            cycles: 0,
            lcd_enabled: false,
            mode: Mode::Hblank,
            line: 0,
            vb_interrupt_enabled: false,
            oam_is_acessed: false,
            line_check: 0,
            lck_interrupt_enabled: false,
            line_equals_line_check: false,
            hb_interrupt_enabled: false,
        }
    }

    fn write_vram(&mut self, address: u16, val: u8) {
        self.vram[address as usize] = val;
    }

    fn oam(&self, address: u16) -> u8 {
        let index = (address / 4) as usize;
        let atrribute = address % 4;

        let sprite = &self.oam[index];

        match atrribute {
            0 => sprite.y,
            1 => sprite.x,
            2 => sprite.tile,
            3 => sprite.flags(),
            _ => unreachable!(),
        }
    }
   

    fn step(&mut self) -> Interrupts {
        let mut request = Interrupts::None;
        if !self.lcd_enabled {
            return request;
        }

       
        let mode = self.mode;
        match mode {
            Mode::Hblank => {
                if self.cycles >= 252 {
                    self.cycles = self.cycles % 252;
                    self.line += 1;

                    if self.line >= 144 {
                        self.mode = Mode::Vblank;
                        request.add(Interrupts::VBlank);
                        if self.vb_interrupt_enabled {
                            request.add(Interrupts::LCDStat)
                        }
                    } else {
                        self.mode = Mode::OAMAccess;
                        if self.oam_is_acessed {
                            request.add(Interrupts::LCDStat)
                        }
                    }
                    self.set_equal_line_check(&mut request);
                }
            }
            Mode::Vblank => {
                if self.cycles >= 456 {
                    self.cycles = self.cycles % 456;
                    self.line += 1;
                    if self.line == 154 {
                        self.mode = Mode::OAMAccess;
                        self.line = 0;
                        if self.oam_is_acessed {
                            request.add(Interrupts::LCDStat);
                        }
                    }
                    self.set_equal_line_check(&mut request);
                }
            }
            Mode::OAMAccess => {
                if self.cycles >= 80 {
                    self.cycles = self.cycles % 80;
                    self.mode = Mode::VRAMAccess;
                }
            }
            Mode::VRAMAccess => {
                if self.cycles >= 43 {
                    self.cycles = self.cycles % 43;
                    if self.hb_interrupt_enabled {
                        request.add(Interrupts::LCDStat)
                    }
                    self.mode = Mode::Hblank;
                    self.render_scan_line()
                }
            }
        }


        request
    }

    fn set_equal_line_check(&mut self, request: &mut Interrupts) {
        let line_equals_line_check = self.line == self.line_check;
        if line_equals_line_check && self.lck_interrupt_enabled {
            request.add(Interrupts::LCDStat);
        }

        self.line_equals_line_check = line_equals_line_check;
    }

    fn render_scan_line(&mut self) {
        todo!()
    }
}
