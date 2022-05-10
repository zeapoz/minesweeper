mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
pub enum Tile {
    Empty,
    Mine,
}

#[wasm_bindgen]
#[repr(u8)]
pub enum TileState {
    Covered,
    Uncovered,
}

#[wasm_bindgen]
pub struct Board {
    width: u32,
    height: u32,
    tiles: Vec<Tile>,
    uncovered: Vec<TileState>,
}

#[wasm_bindgen]
impl Board {
    pub fn new(width: u32, height: u32) -> Board {
        let uncovered = (0..width * height).map(|_| TileState::Covered).collect();
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
            uncovered,
        }
    }

    pub fn uncover_tile(&mut self, row: u32, col: u32) {
        let i = self.get_index(row, col);
        self.uncovered[i] = TileState::Uncovered;
    }

    pub fn tiles(&self) -> *const Tile {
        self.tiles.as_ptr()
    }

    pub fn uncovered(&self) -> *const TileState {
        self.uncovered.as_ptr()
    }
}

impl Board {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }
}
