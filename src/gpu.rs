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