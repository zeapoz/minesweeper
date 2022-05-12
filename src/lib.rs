mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Tile {
    Empty = 0,
    Mine = 1,
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

        let mut board = Board {
            width,
            height,
            tiles,
            uncovered,
            neighbors: vec![],
        };

        board.calculate_neighbors();
        board
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

impl Board {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn calculate_neighbors(&mut self) {
        let mut neighbors = vec![];

        for row in 0..self.height {
            for col in 0..self.width {
                let sum = self.sum_neighbors(row, col);
                neighbors.push(sum);
            }
        }

        self.neighbors = neighbors;
    }

    fn sum_neighbors(&self, row: u32, col: u32) -> u8 {
        let mut sum = 0;
        for delta_row in [-1, 0, 1].iter().cloned() {
            for delta_col in [-1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = row as i32 + delta_row;
                let neighbor_col = col as i32 + delta_col;

                if neighbor_row < 0
                    || neighbor_col < 0
                    || neighbor_row >= self.height as i32
                    || neighbor_col >= self.width as i32
                {
                    continue;
                }

                let idx = self.get_index(neighbor_row as u32, neighbor_col as u32);
                sum += self.tiles[idx] as u8;
            }
        }
        sum
    }
}
