
pub struct Ram {
    byte: Vec<u8>,
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            byte: Vec::with_capacity(0x10000)
        }
    }

    pub fn read_byte(&self, address: usize) -> u8 {
        self.byte[address] 
    }
}