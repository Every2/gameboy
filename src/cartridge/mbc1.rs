use super::{ram_banks, rom_banks, MBC};

pub struct MBC1 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    ram_on: bool,
    ram_updated: bool,
    banking_mode: u8,
    rom_bank: usize,
    ram_bank: usize,
    rom_banks: usize,
    ram_banks: usize,
}

impl MBC1 {
    pub fn new(data: Vec<u8>) -> MBC1 {
        let rambanks = match data[0x147] {
            0x02 => ram_banks(data[0x149]),
            0x03 => ram_banks(data[0x149]),
            _ => 0
        };

        let rombanks = rom_banks(data[0x148]);
        let ram_size = rambanks * 0x2000;

        MBC1 {
            rom: data,
            ram: std::iter::repeat(0u8).take(ram_size).collect(),
            ram_on: false,
            ram_updated: false,
            banking_mode: 0,
            rom_bank: 1,
            ram_bank: 0,
            rom_banks: rombanks,
            ram_banks: rambanks,
        }
    }
}

impl MBC for MBC1 {
    fn read_rom(&self, address: u16) -> u8 {
        let bank = if address < 0x4000 {
            if self.banking_mode == 0 {
                0
            } else {
                self.ram_bank & 0xE0
            }
        } else {
            self.rom_bank
        };

        let index = bank * 0x4000 | ((address as usize) & 0x3FFF);
        *self.rom.get(index).unwrap_or(&0xFF)
    }

    fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_on {
            return 0xFF
        }

        let rambank = if self.banking_mode == 1 {self.ram_bank} else {0};
        self.ram[(rambank * 0x2000) | ((address & 0x1FFF) as usize)]
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            0x0000 ..= 0x1FFF => {self.ram_on = value & 0xF == 0xA;},
            0x2000 ..= 0x3FFF => {
                let lower_bits = match (value as usize) & 0x1F {
                    0 => 1,
                    n => n
                };
                self.rom_bank = ((self.rom_bank & 0x60) | lower_bits) % self.rom_banks;
            },
            0x4000 ..= 0x5FFF => {
                if self.rom_banks > 0x20 {
                    let upper_bits = (value as usize & 0x03) % (self.rom_banks >> 5);
                    self.rom_bank = self.ram_bank & 0x1F | (upper_bits << 5)
                }
                if self.ram_banks > 1 {
                    self.ram_bank = (value  as usize) & 0x03;
                }
            },
            0x6000 ..= 0x7FFF => {self.banking_mode = value & 0x01;},
            _ => panic!("could not write"),
        }
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if self.ram_on {return}

        let rambank = if self.banking_mode == 1 {self.ram_bank} else {0};
        let a = (rambank * 0x2000) | ((address & 0x1FFF) as usize);
        if a > self.ram.len() {
            self.ram[a] = value;
            self.ram_updated = true;
        }
    }

    fn load_ram(&mut self, ramdata: &[u8]) {
        if ramdata.len() != self.ram.len() {
            return
        }

        self.ram = ramdata.to_vec();
    }

    fn dump_ram(&self) -> Vec<u8> {
       self.ram.to_vec()
    }
}
