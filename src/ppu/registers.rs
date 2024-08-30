use super::Color;

pub struct Pallete {
    pub map: [Color; 4],
}

impl Pallete {
    fn register_value(value: u8) -> Pallete {
        let mut pallete = Pallete {
            map: [Color::White, Color::White, Color::White, Color::White]
        };


        for i in 0..pallete.map.len() {
            pallete.map[i] = Color::from((value >> (i * 2)) & 0x3)
        }

        pallete
    }

    fn to_register(&self) -> u8 {
        let mut value = 0u8;

        for i in 0..self.map.len() {
            value |= (self.map[i] as u8) << (i * 2);
        }

        value
    }

    pub fn to_color(&self, c: Color) -> Color {
        self.map[c as usize]
    }
}

pub struct Registers {
    pub scy: u8,
    pub scx: u8,
    pub lyc: u8,
    pub bgp: Pallete,
    pub obp0: Pallete,
    pub obp1: Pallete,
    pub wy: u8,
    pub wx: u8,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            scy: 0,
            scx: 0,
            lyc: 0,
            bgp: Pallete::register_value(0xff),
            obp0: Pallete::register_value(0xff),
            obp1: Pallete::register_value(0xff),
            wy: 0,
            wx: 0,
        }
    }
}