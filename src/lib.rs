pub use life::life::ToroidalGrid;
use crate::video::graphics;

pub mod life;
pub mod console;
pub mod video;

pub async fn emulate(board: &mut ToroidalGrid) {
  loop {
    graphics::graphics(board).await;
    board.update_board();
  }
}

