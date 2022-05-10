mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub enum Tile {
    Empty,
    Mine,
}

#[wasm_bindgen]
pub struct Board {
    width: u32,
    height: u32,
    tiles: Vec<Tile>,
}

#[wasm_bindgen]
impl Board {
    pub fn new(width: u32, height: u32) -> Board {
        let tiles = (0..width * height)
            .map(|i| {
                if i % 2 == 0 && i % 3 != 0 {
                    Tile::Mine
                } else {
                    Tile::Empty
                }
            })
            .collect();

        Board {
            width,
            height,
            tiles,
        }
    }

    pub fn tiles(&self) -> *const Tile {
        self.tiles.as_ptr()
    }
}
