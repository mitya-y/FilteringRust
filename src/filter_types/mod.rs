pub mod negative;
pub use negative::*;

pub mod blur;
pub use blur::*;

pub mod gauss_blur;
pub use gauss_blur::*;

pub mod laplacian;
pub use laplacian::*;

pub trait FilterMatrix {
  fn get3x3(&self) -> ([[i8; 3]; 3], f32);
}
