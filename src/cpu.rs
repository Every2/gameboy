use crate::{hardware::Hardware, ram::Ram, registers::Registers};


pub struct Cpu {
    pub registers: Registers,
    pub pc: u16,
    pub sp: u16,
    pub external: Hardware,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            registers: Registers::new(),
            pc: 0x0,
            sp: 0x00,
            external: Hardware::new(),
        }
    }


    pub fn set_pc(&mut self, pc: u16) {
        self.pc = pc;
    }
}