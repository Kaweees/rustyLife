use std::process;
use macroquad::prelude::*;

use crate::ToroidalGrid;
use crate::life::life::CellOptions;

pub async fn graphics(grid: &mut ToroidalGrid) -> bool {
    clear_background(BLACK);

    if is_quit_requested() { process::exit(-1) }

    draw_screen(grid);
    next_frame().await;

    if is_key_down(KeyCode::D) { return true }
    false
}

fn draw_screen(grid: &mut ToroidalGrid) {
    let w = screen_width();
    let h = screen_height();
    println!("Expected: ({}, {})", grid.width * grid.cell_size, grid.height * grid.cell_size);
    println!("Reality: ({}, {})", w, h);

    // /* Draw cells */
    // for i in 0..(grid.width) {
    //   for j in 0..(grid.height) {
    //     draw_rectangle((i * grid.cell_size) as f32 / w, (j * grid.cell_size) as f32 / h, (grid.cell_size) as f32, (grid.cell_size) as f32,  if grid.get_toroidal_grid(i as isize, j as isize) == CellOptions::ALIVE {GREEN} else {BLACK});
    //   }
    // }

    /* Draw horizontal lines */
    for i in 1..(grid.height) {
      draw_line(0 as f32 / w, (i * grid.cell_size) as f32, w, (i * grid.cell_size) as f32 / h, 1.0, WHITE)
    }

    /* Draw vertical lines */
    for j in 1..(grid.width) {
      draw_line((j * grid.cell_size) as f32, 0.0, (j * grid.cell_size) as f32, (grid.height * grid.cell_size) as f32, 1.0, WHITE)
    }
    
    draw_text("Conway's Game of Life", 1.0, w / 20.0, w / 20.0, WHITE);
}
