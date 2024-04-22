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
    // println!("Expected: ({}, {})", grid.width * grid.cell_size, grid.height * grid.cell_size);
    // println!("Reality: ({}, {})", w, h);
    // println!("{}", (i * grid.cell_size) as f32 / h);
    
    /* Draw cells */
    let mut align_x: f32 = 0.0;
    let mut align_y: f32 = 0.0;
    for i in 0..(grid.width) {
      for j in 0..(grid.height) {
        draw_rectangle(align_x, align_y, w / (grid.width as f32), h / (grid.height as f32),  if grid.get_toroidal_grid(i as isize, j as isize) == CellOptions::ALIVE {GREEN} else {BLACK});
        align_y += h / (grid.height as f32);
      }
      align_x += w / (grid.width as f32);
      align_y = 0.0;
    }
    
    
    /* Draw horizontal lines */
    align_y = 0.0;
    for _i in 0..(grid.height) {
      // println!("{}", align);
      // println!("{}", h / (grid.width as f32));
      // println!("{}", (i * grid.cell_size) as f32);
      draw_line(0 as f32, align_y, w, align_y, 1.0, WHITE);
      align_y += h / (grid.height as f32);
    }

    /* Draw vertical lines */
    align_x = 0.0;
    for _j in 0..(grid.width) {
      draw_line(align_x, 0.0, align_x, h, 1.0, WHITE);
      align_x += w / (grid.width as f32);
    }
}
