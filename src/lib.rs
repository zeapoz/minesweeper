mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

enum Tile {
    Empty(u32),
    Mine,
}

#[wasm_bindgen]
struct Board {
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl Board {
    fn new() -> Board {
        Board {
            width: 10,
            height: 10,
        }
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.width
    }
}
