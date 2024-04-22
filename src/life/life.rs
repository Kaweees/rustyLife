use std::thread::sleep;
use std::time::Duration;

#[derive(Clone, Copy, PartialEq)]
pub enum CellOptions {
  DEAD = 0,
  ALIVE = 1,
}

pub struct ToroidalGrid {
  pub width: usize, // The width of the toroidal grid
  pub height: usize, // The width of the toroidal grid
  pub data: Vec<Vec<CellOptions>>, // 
}

impl ToroidalGrid {
  pub fn new(width: usize, height: usize) -> ToroidalGrid {
    let mut data = vec![vec![CellOptions::DEAD; width]; height];
    for row in &mut data {
      row.resize(width, CellOptions::DEAD);
    }
    ToroidalGrid { width, height, data }
  }

  pub fn display_toroidal_grid(&self) {
    sleep(Duration::from_secs(1));
    print!("\x1B[2J\x1B[1;1H"); // Clear the screen
    for row in &self.data {
      for &cell in row {
          print!("{} ", if cell == CellOptions::ALIVE { '1' } else { '0' });
      }
      println!();
    }
  }

  pub fn get_toroidal_grid(&mut self, x: isize, y: isize) -> CellOptions {
    let x = ((x + self.width as isize) % self.width as isize) as usize;
    let y = ((y + self.height as isize) % self.height as isize) as usize;
    self.data[y][x]
  }

  pub fn set_toroidal_grid(&mut self, x: isize, y: isize, value: CellOptions) {
    let x = ((x + self.width as isize) % self.width as isize) as usize;
    let y = ((y + self.height as isize) % self.height as isize) as usize;
    self.data[y][x] = value;
  }  

  pub fn count_alive_neighbors(&mut self, x: isize, y: isize) -> u8 {
    let mut count: u8 = 0;
    let lower_bound: isize = -1;
    for i in -1..=1 {
      for j in -1..=1 {
          if i == 0 && j == 0 {
              continue;
          }
          if self.get_toroidal_grid((x as isize) + i, (y as isize) + j) == CellOptions::ALIVE {
              count += 1;
          }
      }
    }
    count
  }
}
