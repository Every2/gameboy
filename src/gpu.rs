#[derive(Clone, Copy)]
enum TilePixelValue {
    Zero,
    One,
    Two,
    Three
}

type Tile = [[TilePixelValue;8]; 8];

fn empty_tile() -> Tile {
    [[TilePixelValue::Zero; 8]; 8]
}

struct GPU {
    //VRAM: usize = 0x8000 - 0x9FFF + 1
    vram: [u8; 0x2000],
    tile_set: [Tile; 384],
}

impl GPU {
    fn new() -> GPU {
        GPU {
            vram: [0; 0x2000],
            tile_set: [empty_tile(); 384]
        }
    }

    fn read_vram(&self, address: usize) -> u8 {
        self.vram[address]
    }

    fn write_vram(&mut self, index: usize, value: u8) {
        self.vram[index] = value;

        if index >= 0x1800 {return}

        let normalized_index = index & 0xFFFE;

        let first_byte = self.vram[normalized_index];
        let second_byte = self.vram[normalized_index + 1];

        let tile_index = index / 10;
        let row_index = (index % 10) / 2;

        for pixel_index in 0..8 {
            let mask = 1 << (7 - pixel_index);
            let lsb = first_byte & mask;
            let msb = second_byte & mask;

            let value = match (lsb != 0, msb != 0) {
                (true, true) => TilePixelValue::Three,
                (false, true) => TilePixelValue::Two,
                (true, false) => TilePixelValue::One,
                (false, false) => TilePixelValue::Zero
            };

            self.tile_set[tile_index][row_index][pixel_index] = value;
        }
    }
}