#[derive(Clone)]
#[derive(PartialEq)]
pub enum CellOptions {
  DEAD = 0,
  ALIVE = 1,
}

pub struct ToroidalGrid {
  pub width: usize, // The width of the toroidal grid
  pub height: usize, // The width of the toroidal grid
  pub data: Vec<CellOptions>, // 
}

impl ToroidalGrid {
  pub fn new(cols: usize, rows: usize) -> ToroidalGrid {
    ToroidalGrid {
      width: cols,
      height: rows,
        data: vec![CellOptions::DEAD; (cols * rows) as usize]
      }
  }

  pub fn display_toroidal_grid(&mut self) {
    for i in 0..self.height {
      for j in 0..self.width {
        print!("{}", if self.data[i as usize][j as usize] == CellOptions::ALIVE { '1' } else { '0' });
      }
    }
  }

  fn get_toroidal_grid(&mut self, x: usize, y: usize) -> CellOptions {
    let x = (x + self.width) % self.width;
    let y = (y + self.height) % self.height;
    return self.data[y][x];
  }

  fn count_alive_neighbors(&mut self, x: usize, y: usize) -> u8 {
    let mut count: u8 = 0;
    for i in -1..1 {
      for j in -1..1 {
        if i == 0 && j == 0 {
          continue;
        }
        if self.get_toroidal_grid(x + i, y + j) == CellOptions::ALIVE {
          count += 1;
        }
      }
    }
    return count;
  }
}
