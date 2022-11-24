use crate::{CELL_SIZE, GRID_HEIGHT, GRID_WIDTH};
use macroquad::window::Conf;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Rust life game - Adrien Masanet".to_owned(),
        window_width: (GRID_WIDTH * CELL_SIZE as i32) as i32,
        window_height: (GRID_HEIGHT * CELL_SIZE as i32) as i32,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}
