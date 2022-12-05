const CELL_SIZE: usize = 5; // Cell size in pixels
const CELL_SIZE_AS_F32: f32 = CELL_SIZE as f32;
const CELL_COUNT_START: usize = 2000; // Number of cells to start with
const GRID_WIDTH: i32 = 150; // Grid width in cells
const GRID_HEIGHT: i32 = 150; // Grid height in cells
const TIME_TO_SLEEP_BETWEEN_CYCLES: u64 = 1; // Time to sleep between cycles in milliseconds, can be used to slow down the simulation if it's running too fast

mod cell;
mod grid;
mod window;

use grid::Grid;
use macroquad::prelude::*;
use std::{thread, time};
use window::window_conf;

#[macroquad::main(window_conf)] // Instantiating the window with the window_conf function thanks to the macroquad crate
async fn main() {
    // Create and populate the grid that will contain the cells
    let mut grid: Grid = Grid::new();
    grid.populate_cells();

    loop {
        clear_background(BLACK);

        grid.update();

        next_frame().await;

        thread::sleep(time::Duration::from_millis(TIME_TO_SLEEP_BETWEEN_CYCLES));
    }
}
