pub struct GaussBlurFilter;

impl super::FilterMatrix for GaussBlurFilter {
    fn get3x3(&self) -> ([[i8; 3]; 3], f32) {
      (
        [
          [1, 2, 1],
          [2, 4, 2],
          [1, 2, 1]
        ],
        1.0 / 16.0
      )
    }
  }
  