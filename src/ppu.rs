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

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Hblank,
    Vblank,
    Prelude,
    Active,
}

impl std::convert::From<Mode> for u8 {
    fn from(value: Mode) -> Self {
        match value {
            Mode::Hblank => 0,
            Mode::Vblank => 1,
            Mode::Prelude => 2,
            Mode::Active => 3,
        }
    }
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

    fn top_line(&self) -> i8 {
        (self.y as i8) - 16
    }

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
    sprite: [Sprite; 40],
    oam: [u8; 0x100],
}
