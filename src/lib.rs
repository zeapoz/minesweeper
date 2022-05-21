mod utils;

use rand::{thread_rng, Rng};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Empty = 0,
    Mine = 1,
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(PartialEq)]
pub enum TileState {
    Covered,
    Uncovered,
    Flagged,
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
        let mut rng = thread_rng();
        let uncovered = (0..width * height).map(|_| TileState::Covered).collect();
        let tiles = (0..width * height)
            .map(|_| {
                if rng.gen_range(0.0, 1.0) > 0.8 {
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
        if self.uncovered[i] != TileState::Covered {
            return;
        }

        self.uncovered[i] = TileState::Uncovered;

        // Flood fill neighbors if no mines are nearby
        if self.neighbors[i] == 0 && self.tiles[i] != Tile::Mine {
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

                    self.uncover_tile(neighbor_row as u32, neighbor_col as u32);
                }
            }
        }
    }

    pub fn flag_tile(&mut self, row: u32, col: u32) {
        let i = self.get_index(row, col);

        self.uncovered[i] = match self.uncovered[i] {
            TileState::Covered => TileState::Flagged,
            TileState::Uncovered => return,
            TileState::Flagged => TileState::Covered,
        }
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
