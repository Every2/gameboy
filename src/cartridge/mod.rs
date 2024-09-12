mod mbc0;
mod mbc1;

pub trait MBC {
    fn read_rom(&self, address: u16) -> u8;
    fn read_ram(&self, address: u16) -> u8;
    fn write_rom(&mut self, address: u16, value: u8);
    fn write_ram(&mut self, address: u16, value: u8);
    fn load_ram(&mut self, ramdata: &[u8]);
    fn dump_ram(&self) -> Vec<u8>;
    fn rom_name(&self) -> String {
        const TITLE: u16  = 0x134;
        const CGB_FLAG: u16 = 0x143;

        let title_size = match self.read_rom(CGB_FLAG) & 0x80 {
            0x80 => 11,
            _ => 16,
        };

        let mut result = String::with_capacity(title_size);

        for i in 0..title_size {
            match self.read_rom(TITLE + (i as u16)) {
                0 => break,
                value => result.push(value as char)
            }
        }

        result
    }
}

fn ram_banks(value: u8) -> usize {
    match value {
        1 => 1,
        2 => 1,
        3 => 4,
        4 => 16,
        5 => 8,
        _ => 0,
    }
}

fn rom_banks(value: u8) -> usize {
    if value  <= 8 {
        2 << value
    } else {
        0
    }
}
