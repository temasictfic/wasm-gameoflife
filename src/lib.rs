// region: use
mod utils;

use js_sys::Math;
use wasm_bindgen::prelude::*;
use web_sys::console;

use fixedbitset::FixedBitSet;
use std::collections::HashSet;

#[allow(unused_macros)]
// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        #[allow(unused_unsafe)]
        unsafe{console::log_1(&format!( $( $t )* ).into())};
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
// endregion

// region: Timer
pub struct Timer<'a> {
    name: &'a str,
}
#[allow(unused_unsafe)]
impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        unsafe {
            console::time_with_label(name);
        }
        Timer { name }
    }
}
#[allow(unused_unsafe)]
impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        unsafe {
            console::time_end_with_label(self.name);
        }
    }
}
// endregion

/* enum Cell
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}*/

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    prevs: FixedBitSet,
    nexts: FixedBitSet,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let north = if row == 0 { self.height - 1 } else { row - 1 };

        let south = if row == self.height - 1 { 0 } else { row + 1 };

        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };

        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let cords = [
            (north, west),
            (north, column),
            (north, east),
            (row, west),
            (row, east),
            (south, west),
            (south, column),
            (south, east),
        ];

        let count = cords
            .iter()
            .map(|&(x, y)| self.prevs[self.get_index(x, y)] as u8)
            .sum();

        /* for loop
        let mut count = 0;
        for &(x, y) in cords.iter() {
            let idx = self.get_index(x, y);
            count += self.prevs[idx] as u8;
        }*/

        count
    }

    /* old live_neighbor_count
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.prevs[idx] as u8;
            }
        }
        count
    }
    */

    // region: for testing
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.nexts
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for &(row, col) in cells.iter() {
            let idx = self.get_index(row, col);
            self.prevs.set(idx, true);
            self.nexts.set(idx, true);
        }
    }
    // endregion
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        //let _timer = Timer::new("Universe::tick");

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.prevs[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                /*
                log!(
                    "cell[{}, {}] is initially {:?} and has {} live neighbors",
                    row,
                    col,
                    cell,
                    live_neighbors
                );*/

                match (cell, live_neighbors) {
                    (true, x) if x < 2 => self.nexts.set(idx, false),
                    (true, x) if x > 3 => self.nexts.set(idx, false),
                    (false, 3) => self.nexts.set(idx, true),
                    (_, _) => (),
                };

                /* log!("    it becomes {:?}", next);


                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other prevs remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;*/
            }
        }

        /* log changed
        log!(
            "prevs(row, col) : {:?} has changed state.",
            {
                let dif = self.prevs.symmetric_difference(&next).collect::<FixedBitSet>();
                dif.ones().map(|idx| { let row = idx as u32 / self.width;
                                        let col = idx as u32 % self.width;
                                        (row, col) }).collect::<Vec<_>>()
            }

        );*/

        self.prevs.clear();
        self.prevs.union_with(&self.nexts);
    }
    #[allow(unused_unsafe)]
    pub fn new(width: u32, height: u32) -> Universe {
        utils::set_panic_hook();

        let size = (width * height) as usize;
        let mut prevs = FixedBitSet::with_capacity(size);
        let mut nexts = FixedBitSet::with_capacity(size);

        for idx in 0..size {
            prevs.set(idx, unsafe { Math::random() < 0.33 });
        }

        nexts.union_with(&prevs);

        /* old prevs
        let prevs = (0..size)
            .map(|_| {
                if js_sys::Math::random() < 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();*/

        Universe {
            width,
            height,
            prevs,
            nexts,
        }
    }
    pub fn new_empty(width: u32, height: u32) -> Universe {
        utils::set_panic_hook();

        let size = (width * height) as usize;
        let prevs = FixedBitSet::with_capacity(size);
        let nexts = FixedBitSet::with_capacity(size);

        Universe {
            width,
            height,
            prevs,
            nexts,
        }
    }

    /* Canvas api kullanınca bu ve impl fmt::Display gereksiz kaldı.
    pub fn render(&self) -> String {
        self.to_string()
    }*/

    pub fn set_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.prevs.set(idx, true);
        self.nexts.set(idx, true);
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.prevs.toggle(idx);
        self.nexts.toggle(idx);
    }

    pub fn glider(&mut self, row: u32, col: u32) {
        if row > 0 && row < self.height && col > 0 && col < self.width {
            let points = [
                (row - 1, col),
                (row, col + 1),
                (row + 1, col - 1),
                (row + 1, col),
                (row + 1, col + 1),
            ];

            for &(r, c) in points.iter() {
                let idx = self.get_index(r, c);
                self.prevs.set(idx, true);
                self.nexts.set(idx, true);
            }
        }
    }

    pub fn pulsar(&mut self, row: u32, col: u32) {
        if row > 6 && row < self.height - 6 && col > 6 && col < self.width - 6 {
            let mut points: HashSet<usize> = HashSet::new();
            let quarter = [
                (6, 4),
                (6, 3),
                (6, 2),
                (2, 1),
                (3, 1),
                (4, 1),
                (1, 2),
                (1, 3),
                (1, 4),
                (2, 6),
                (3, 6),
                (4, 6),
            ];
            for &(r, c) in quarter.iter() {
                points.insert(self.get_index(row + r, col + c));
                points.insert(self.get_index(row + r, col - c));
                points.insert(self.get_index(row - r, col + c));
                points.insert(self.get_index(row - r, col - c));
            }

            for idx in points {
                self.prevs.set(idx, true);
                self.nexts.set(idx, true);
            }
        }
    }

    /* getters
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }*/

    pub fn nexts(&self) -> *const u32 {
        self.nexts.as_slice().as_ptr()
    }

    /* new_empty oluşturunca gereksiz oldular.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        for i in 0..self.height * width {
            self.prevs.set(i as usize, false);
        }
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        for i in 0..self.width * height {
            self.prevs.set(i as usize, false);
        }
    } */
}

/* Display for Universe
use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.prevs.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == 0 { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}*/
