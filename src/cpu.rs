use crate::registers::Registers;

struct Cpu {
    registers: Registers,
    pc: u16,
    sp: u16,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            registers: Registers::new(),
            pc: 0x0,
            sp: 0x00,
        }
    }
}