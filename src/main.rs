use macroquad::window::Conf;
use rusty_life;
use console::*;
use std::process;

mod console;

use crate::rusty_life::ToroidalGrid;
// use rusty_life::ToroidalGrid;

fn window_conf() -> Conf {
    Conf {
        window_title: "Conway's Game of Life".to_owned(),
        window_width: (CELL_SIZE * GRID_WIDTH) as i32,
        window_height: (CELL_SIZE * GRID_HEIGHT) as i32,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
  let mut board = ToroidalGrid::new(GRID_WIDTH, GRID_HEIGHT, CELL_SIZE);
  rusty_life::emulate(&mut board).await;
  process::exit(EXIT_SUCCESS);
}

