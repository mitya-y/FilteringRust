pub mod negative;
pub use negative::*;

pub mod blur;
pub use blur::*;

pub trait FilterMatrix {
  fn get3x3(&self) -> ([[u8; 3]; 3], f32);
}
