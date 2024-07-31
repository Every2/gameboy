
pub struct Ram {
    byte: Vec<u8>,
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            byte: Vec::with_capacity(0x10000)
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.byte[address as usize] 
    }

    pub fn store_byte(&mut self, address: u16, value: u8) {
        self.byte[address as usize] = value;
    }
}