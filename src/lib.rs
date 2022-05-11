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
    neighbors: Vec<u8>,
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

        let neighbors = calculate_neighbors(&tiles, width, height);

        Board {
            width,
            height,
            tiles,
            uncovered,
            neighbors,
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

    pub fn neighbors(&self) -> *const u8 {
        self.neighbors.as_ptr()
    }
}

fn calculate_neighbors(board: &Vec<Tile>, width: u32, height: u32) -> Vec<u8> {
    let mut neighbors = vec![];

    for i in 0..(width * height) {
        let sum = sum_neighbors(board, i, width, height);
        neighbors.push(sum);
    }

    neighbors
}

fn sum_neighbors(board: &Vec<Tile>, index: u32, width: u32, height: u32) -> u8 {
    0
}

impl Board {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }
}
