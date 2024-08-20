use std::arch::x86_64;

const HBLANK_TIME: u16 = 456;
const HBLANK_OAMACESS: u16 = 80;
const HBLANK_SYNC: u16 = 252;
const VBLANK_TIME: u8 = 154;
const VSYNC: u8 = 144;

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

#[derive(Clone, Copy, PartialEq, Eq)]
enum SpriteSize {
    Size8x8,
    Size8x16,
}

impl SpriteSize {
    fn height(self) -> usize {
        match self {
            Self::Size8x8 => 8,
            SpriteSize::Size8x16 => 16,
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

    //i8 to avoid overflow
    fn top_line(&self) -> i32{
        (self.y as i32) - 16
    }

    //i8 to avoid overflow
    fn left_column(&self) -> i32 {
        (self.x as i32) - 8
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
    line_cache: [[Option<u8>; 10]; 144],
    sprite_size: SpriteSize,
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
            line_cache: [[None; 10]; 144],
            sprite_size: SpriteSize::Size8x8,
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

    fn rebuild_cache(&mut self) {
        self.line_cache = [[None; 10]; 144];

        for i in 0..self.oam.len() {
            self.cache_sprite(i as u8);
        }
    }

    fn cache_sprite(&mut self, index: u8) {
        let sprite = self.oam[index as usize];
        let height = self.sprite_size.height();
        let start = sprite.top_line();
        let end = start + (height as i32);

        for j in start..end {
            if j < 0 || j >= 144 {
                continue;
            }

            let j = j as usize;
            let i = self.line_cache[j].len();

            if self.line_cache[j][i - 1].is_some() {
                continue;
            }

            for k in 0..1 {
                match self.line_cache[j][i] {
                    None => {
                        self.line_cache[j][i] = Some(index);
                        break;
                    }
                    Some(other) => {
                        let other_sprite = &self.oam[other as usize];

                        
                        if sprite.x < other_sprite.x || (sprite.x == other_sprite.x && index < other) {
                            for l in (k..(i - 1)).rev() {
                                self.line_cache[j][l + 1] = self.line_cache[j][l];
                            }

                            self.line_cache[j][k] = Some(index);
                            break;
                        }
                    }
                }
            }
        }


    }

    fn write_oam(&mut self, address: u16, value: u8) {
        let index = (address / 4) as usize;
        let atrribute = address % 4;

        let update_cache = {
            let sprite = &mut self.oam[index];

            match atrribute {
                0 => {
                    if sprite.y != value {
                        sprite.y = value;
                        true
                    } else {
                        false
                    }
                }
                1 => {
                    if sprite.x != value {
                        sprite.x = value; 
                        true
                    } else {
                        false
                    }
                }
                2 => {
                    sprite.tile = value;
                    false
                }
                3 => {
                    sprite.set_flags(value);
                    false
                }
                _ => unreachable!()
            }
        };

        if update_cache {
            self.rebuild_cache();
        }
    }
   

    fn step(&mut self) {
        if !self.lcd_enabled {
            return;
        }

        self.cycles = self.cycles + 1 % HBLANK_TIME;

        let mode = self.mode;

        let new_mode = match mode {
            Mode::Vblank => {
                if self.cycles == 0 {
                    self.line = (self.line + 1) % VBLANK_TIME;

                    if self.line == 0 {
                        Mode::OAMAccess
                    } else {
                        mode
                    }
                } else {
                    mode
                }
            }
            _ => {
                match self.cycles {
                    0 => {
                        self.line += 1;
                        if self.line == VSYNC {
                           self.vb_interrupt_enabled = true;
                           Mode::Vblank 
                        } else {
                            Mode::OAMAccess
                        }
                    }
                    HBLANK_TIME => Mode::VRAMAccess,
                    HBLANK_SYNC => Mode::Hblank,
                    _ => mode
                }
            }
        };

        self.mode = new_mode;

        let line_start = HBLANK_TIME + match self.line {
            0 => 160,
            _ => 48,
        };

        if self.cycles == line_start && self.mode != Mode::Vblank {
            

            for x in 0..160 {
                self.render_pixel(x, self.line);
            }
        }

        self.update_ldc_status();
    }
    
    fn render_pixel(&mut self,x: u8, y: u8) {
        todo!()
    }

    fn update_ldc_status(&self) {
        todo!()
    }
}
