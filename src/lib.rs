pub use life::life::ToroidalGrid;
use crate::video::graphics;

mod life;
mod console;
mod video;

pub async fn emulate(board: &mut ToroidalGrid) {
  loop {
    graphics::graphics(board).await;
    board.update_board();
  }
}

