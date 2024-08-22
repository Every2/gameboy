use super::Color;

struct Pallete {
    map: [Color; 4],
}

struct Registers {
    lcdc: u8,
    scy: u8,
    scx: u8,
    lyc: u8,
    bgp: u8,
    obp0: Pallete,
    obp1: Pallete,
    wy: u8,
    wx: u8, 
}