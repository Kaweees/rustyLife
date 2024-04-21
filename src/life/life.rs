pub enum CellOptions {
  DEAD = 0,
  ALIVE = 1,
}

pub struct ToroidalGrid {
  pub width: u32, // The width of the toroidal grid
  pub height: u32, // The width of the toroidal grid
  pub data: Vec<CellOptions>, // 
}

impl ToroidalGrid {
  fn new(cols: u32, rows: u32) -> ToroidalGrid {
      Matrix {
          cols: cols,
          rows: rows,
          data: vec![DEAD; cols * rows]
      }
  }

  fn displayToroidalGrid() -> None {
    for i in 0..grid.height {
      for j in 0..grid.width {
        print!("{}", if grid.data[i][j] { '1' } else { '0' });
      }
    }
  }

  fn getToroidalGrid(x: u32, y: u32) -> CellOptions {
    x = (x + grid.width) % grid.width;
    y = (y + grid.height) % grid.height;
    return grid.data[y][x];
  }

  fn countAliveNeighbors(cols: u32, rows: u32) -> u32 {
    let mut count: u32 = 0;
    for i in -1..1 {
      for j in -1..1 {
        if (i == 0 && j == 0) {
          continue;
        }
        if (getToroidalGrid(grid, x + i, y + j) == ALIVE) {
          count += 1;
        }
      }
    }
    return count;
  }
}
