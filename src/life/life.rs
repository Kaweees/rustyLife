use std::thread::sleep;
use std::time::Duration;

#[derive(Clone, Copy, PartialEq)]
pub enum CellOptions {
  DEAD = 0,
  ALIVE = 1,
}

#[derive(Clone)]
pub struct ToroidalGrid {
  pub width: usize, // The width of the toroidal grid
  pub height: usize, // The width of the toroidal grid
  pub cell_size: usize, // The size of cells in the toroidal grid
  pub data: Vec<Vec<CellOptions>>, // 
}

impl ToroidalGrid {
  pub fn new(width: usize, height: usize, cell_size: usize) -> ToroidalGrid {
    let mut data = vec![vec![CellOptions::DEAD; width]; height];
    for row in &mut data {
      row.resize(width, CellOptions::DEAD);
    }
    ToroidalGrid { width, height, cell_size, data }
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

  pub fn update_board(&mut self) {
    let mut cloned_grid: ToroidalGrid = self.clone();
    for i in 0..(self.width) {
      for j in 0..(self.height) {
        let alive_neighbors: u8 = cloned_grid.count_alive_neighbors(j as isize, i as isize);
        if cloned_grid.data[i][j] == CellOptions::ALIVE {
          /* If an ALIVE cell has 2 or 3 ALIVE neighbors, it will be ALIVE in the
           * next time step. Otherwise, it will be DEAD.*/
          if alive_neighbors == 2 || alive_neighbors == 3 {
            self.data[i][j] = CellOptions::ALIVE;
          } else {
            self.data[i][j] = CellOptions::DEAD;
          }
        } else {
          /* If a DEAD cell has 3 ALIVE neighbors, it will be ALIVE in the next
            * time step. */
          if alive_neighbors == 3 {
            self.data[i][j] = CellOptions::ALIVE;
          } else {
            self.data[i][j] = CellOptions::DEAD;
          }
        }
      }
    }

  }
}
