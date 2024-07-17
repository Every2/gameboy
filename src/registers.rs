pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

impl Registers {
    fn af(&self) -> u16 {
        ((self.a as u16) << 8)| ((self.f & 0xF0) as u16)
    }

    fn bc(&self) -> u16 {
        ((self.b as u16) << 8)| (self.c as u16)
    }

    fn de(&self) -> u16 {
        ((self.d as u16) << 8)| (self.e as u16)
    }

    fn hl(&self) -> u16 {
        ((self.h as u16) << 8)| (self.l as u16)
    }

    fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0x00F0) as u8;
    }

    fn set_bc(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0x00FF) as u8;
    }

    fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }
}