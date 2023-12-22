pub struct BlurFilter;

impl super::FilterMatrix for BlurFilter {
    fn get3x3(&self) -> ([[u8; 3]; 3], f32) {
      (
        [
          [1, 1, 1],
          [1, 1, 1],
          [1, 1, 1]
        ],
        1.0 / 9.0
      )
    }
  }
  