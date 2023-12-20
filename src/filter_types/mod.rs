pub mod negative;
pub use negative::*;

pub trait FilterMatrix {
  fn get3x3(&self) -> Vec<Vec<f32>>;
}
