use super::MBC;

pub struct MBC0 {
    rom: Vec<u8>,
}

impl MBC0 {
    pub fn new(data: Vec<u8>) -> MBC0 {
        MBC0 {
            rom: data,
        }
    }
}

impl MBC for MBC0 {
    fn read_rom(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn read_ram(&self, _address: u16) -> u8 {
        0
    }

    fn write_rom(&mut self, _address: u16, _value: u8) {
        ()
    }

    fn write_ram(&mut self,_address: u16, _value: u8) {
        ()
    }

    fn load_ram(&mut self, _ramdata: &[u8]) {
        ()
    }

    fn dump_ram(&self) -> Vec<u8> {
        Vec::new()
    }
}
