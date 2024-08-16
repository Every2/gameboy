#[derive(Clone, Copy)]
enum TilePixelValue {
    Zero,
    One,
    Two,
    Three
}

type Tile = [[TilePixelValue; 8]; 8];

fn empty_tile() -> Tile {
    [[TilePixelValue::Zero; 8]; 8]
}

#[derive(Clone, Copy)]
enum Pallete {
    Zero, 
    One
}

#[derive(Eq, PartialEq)]
enum Interrupts {
    None, 
    VBlank,
    LCDStat,
    Both,
}

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Horizontal,
    Vertical,
    OAM,
    VRAM
}

impl std::convert::From<Mode> for u8 {
    fn from(value: Mode) -> Self {
        match value {
            Mode::Horizontal => 0,
            Mode::Vertical => 1,
            Mode::OAM => 2,
            Mode::VRAM => 3,
        }
    }
}

impl Interrupts {
    fn add(&mut self, other: Interrupts) {
        match self {
            Interrupts::None => *self = other,
            Interrupts::VBlank if other == Interrupts::LCDStat => {
                *self = Interrupts::Both
            }
            Interrupts::LCDStat if other == Interrupts::VBlank => {
                *self = Interrupts::Both
            }
            _ => {}
        };
    }
}

#[derive(Clone, Copy)]
enum TileMap {
    X9800,
    X9C00,
}

#[derive(Clone, Copy)]
struct Sprite {
    x: u8,
    y: u8,
    tile: u8,
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
            x_flip: false,
            y_flip: false,
            priority: false,
            pallete: Pallete::Zero
        }
    }
}


struct GPU {
    //VRAM: usize = 0x8000 - 0x9FFF + 1
    vram: [u8; 0x2000],
    tile_set: [Tile; 384],
    //Gameboy ppu can display 40 objects (sprites)
    sprite: [Sprite; 40],
    oam: [u8; 0xA0],
    window_tile_map: TileMap,
    background_tile_map: TileMap,
    cycles: u16,
    lcd_display_enabled: bool,
    draw_mode: Mode,
    line: u8,
    vblank_interrupt_enabled: bool,
    oam_interrupt_enabled: bool,
    line_check: u8,
    line_equals_line_check_interrupt_enabled: bool,
    hblank_interrupt_enabled: bool,
}

impl GPU {
    fn new() -> GPU {
        GPU {
            vram: [0; 0x2000],
            tile_set: [empty_tile(); 384],
            sprite: [Sprite::new(); 40],
            oam: [0; 0xA0],
            window_tile_map: TileMap::X9800,
            background_tile_map: TileMap::X9800,
            cycles: 0,
            lcd_display_enabled: false,
            draw_mode: Mode::Horizontal,
            line: 0,
            vblank_interrupt_enabled: false,
            oam_interrupt_enabled: false,
            line_check: 0,
            line_equals_line_check_interrupt_enabled: false,
            hblank_interrupt_enabled: false,
        }
    }

    fn read_vram(&self, address: usize) -> u8 {
        self.vram[address]
    }

    fn write_vram(&mut self, index: usize, value: u8) {
        self.vram[index] = value;

        if index >= 0x1800 {return}

        let normalized_index = index & 0xFFFE;

        let first_byte = self.vram[normalized_index];
        let second_byte = self.vram[normalized_index + 1];

        let tile_index = index / 10;
        let row_index = (index % 10) / 2;

        for pixel_index in 0..8 {
            let mask = 1 << (7 - pixel_index);
            let lsb = first_byte & mask;
            let msb = second_byte & mask;

            let value = match (lsb != 0, msb != 0) {
                (true, true) => TilePixelValue::Three,
                (false, true) => TilePixelValue::Two,
                (true, false) => TilePixelValue::One,
                (false, false) => TilePixelValue::Zero
            };

            self.tile_set[tile_index][row_index][pixel_index] = value;
        }
    }

    fn write_oam(&mut self, index: usize, value: u8) {
        self.oam[index] = value;
        let object_index = index / 4;
        //Gameboy ppu can display 40 objects (sprites)
        if object_index > 40 {return};

        let byte = index % 4;

        let data = self.sprite.get_mut(object_index).unwrap();
        match byte {
            0 => data.y = value,
            1 => data.x = value,
            2 => data.tile = value,
            _ => {
                data.pallete = if (value & 0x10) != 0 {
                    Pallete::One
                } else {
                    Pallete::Zero
                };
                data.x_flip = (value & 0x20) != 0;
                data.y_flip = (value & 0x40) != 0;
                data.priority = (value & 0x80) == 0;
            }
        }
    }

    fn step(&mut self, cycles: u8) -> Interrupts {
        let mut request = Interrupts::None;
        if !self.lcd_display_enabled {
            return request;
        }
        self.cycles += cycles as u16;
        let mode = self.draw_mode;
        match mode {
            Mode::Horizontal => {
                if self.cycles >= 200 {
                    self.cycles = self.cycles % 200;
                    self.line += 1;

                    if self.line >= 144 {
                        self.draw_mode = Mode::Vertical;
                        request.add(Interrupts::VBlank);
                        if self.vblank_interrupt_enabled {
                            request.add(Interrupts::LCDStat)
                        }
                    } else {
                        self.draw_mode = Mode::OAM;
                        if self.oam_interrupt_enabled {
                            request.add(Interrupts::LCDStat)
                        }
                    }
                    self.set_equal_lines_check(&mut request);
                }
            }
            Mode::Vertical => {
                if self.cycles >= 456 {
                    self.cycles = self.cycles % 456;
                    self.line += 1;
                    if self.line == 154 {
                        self.draw_mode = Mode::OAM;
                        self.line = 0;
                        if self.oam_interrupt_enabled {
                            request.add(Interrupts::LCDStat)
                        }
                    }
                    self.set_equal_lines_check(&mut request);
                }
            }
            Mode::OAM => {
                if self.cycles >= 80 {
                    self.cycles = self.cycles % 80;
                    self.draw_mode = Mode::VRAM;
                }
            }
            Mode::VRAM => {
                if self.cycles >= 172 {
                    self.cycles = self.cycles % 172;
                    if self.hblank_interrupt_enabled {
                        request.add(Interrupts::LCDStat)
                    }
                    self.draw_mode = Mode::Horizontal;
                }
            }
        }
        request
    }

    fn set_equal_lines_check(&mut self, request: &mut Interrupts) {
        let lines = self.line == self.line_check;
        if lines && self.line_equals_line_check_interrupt_enabled {
            request.add(Interrupts::LCDStat);
        }
        self.line_equals_line_check_interrupt_enabled = lines;
    }
    
}