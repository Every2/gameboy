mod bit_operations;
mod instructions;
mod registers;
use registers::Registers;
use crate::hardware::Hardware;

pub struct Cpu {
    pub registers: Registers,
    pub pc: u16,
    pub sp: u16,
    pub interrupts: bool,
    pub external: Hardware,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            registers: Registers::new(),
            pc: 0x0,
            sp: 0x00,
            interrupts: true,
            external: Hardware::new(),
        }
    }


    pub fn set_pc(&mut self, pc: u16) {
        self.pc = pc;
    }

    pub fn set_sp(&mut self, sp: u16) {
        self.sp = sp;
    }

    pub fn load_pc(&mut self, pc: u16) {
        self.set_pc(pc);
    }

    pub fn pop_sp(&mut self) -> u16 {
        let mut value = self.external.ram.read_byte(self.sp) as u16;
        self.set_sp(self.sp + 1);
        value |= (self.external.ram.read_byte(self.sp) as u16) << 8;
        self.set_sp(self.sp + 1);
        value
    }

    pub fn push_sp(&mut self, value: u16){
        self.set_sp(self.sp - 1);
        self.external.ram.store_byte(self.sp, (value >> 8) as u8);
        self.set_sp(self.sp - 1);
        self.external.ram.store_byte(self.sp, (value & 0xF0) as u8);
    }
}