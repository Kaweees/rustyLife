use macroquad::window::Conf;
mod life;
use life::life::ToroidalGrid;
// use crate::life::life::ToroidalGrid;

use std::process;

const EXIT_SUCCESS: i32 = 0;
// const EXIT_FAILURE: i32 = 1;

const CELL_SIZE: u32 = 60;
const GRID_WIDTH: u32 = 30;
const GRID_HEIGHT: u32 = 30;

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
  
  let board = ToroidalGrid::new(GRID_WIDTH, GRID_HEIGHT); 
  // rusty_life::emulate(state, io).await;
  process::exit(EXIT_SUCCESS);
}

