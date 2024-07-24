const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

#[derive(Clone, Copy, PartialEq, Debug)]
struct FlagRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool
}


impl std::convert::From<FlagRegister> for u8 {
    fn from(flag: FlagRegister) -> u8 {
        (if flag.zero {1} else {0}) << ZERO_FLAG_BYTE_POSITION |
        (if flag.subtract {1} else {0}) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flag.half_carry {1} else {0}) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry {1} else {0}) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagRegister {
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}

impl FlagRegister {
    pub fn new() -> FlagRegister {
        FlagRegister { zero: false, subtract: false, half_carry: false, carry: false }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagRegister,
    h: u8,
    l: u8,
}

impl Registers {
    pub fn new() -> Registers {
        Registers { a: 0, b: 0, c: 0, d: 0, e: 0, f: FlagRegister::new(), h: 0, l: 0 }
    }

    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8)| u8::from(self.f) as u16
    }

    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8)| (self.c as u16)
    }

    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8)| (self.e as u16)
    }

    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8)| (self.l as u16)
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = FlagRegister::from((value & 0xFF) as u8);
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_set_bc() {
        let mut registers = Registers::new();
        registers.set_bc(0xAFCC);
        assert_eq!(registers.b, 0xAFu8);
        assert_eq!(registers.c, 0xCCu8);    
    }

    #[test]
    fn can_set_f_as_u8() {
        let mut registers = Registers::new();
        let value = 0xC0;
        registers.f = value.into();
        let result: u8 = registers.f.into();
        assert_eq!(result, value);
    }

    #[test]
    fn can_set_f_as_flags_struct() {
        let mut registers = Registers::new();
        let value: FlagRegister = 0xC0.into();
        registers.f = value;
        assert_eq!(registers.f, value);
    }

    #[test]
    fn can_be_converted_to_u8() {
        let mut flags = FlagRegister::new();
        flags.zero = true;
        flags.carry = true;
        let result: u8 = flags.into();
        assert_eq!(result, 0x90u8);
    }

    #[test]
    fn can_be_converted_from_u8() {
        let result: FlagRegister = 0x90.into();
        assert!(result.zero);
        assert!(result.carry);
        assert!(!result.half_carry);
        assert!(!result.subtract);
    }
}