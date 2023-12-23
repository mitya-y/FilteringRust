pub struct NegativeFilter;

impl super::FilterMatrix for NegativeFilter {
  fn get3x3(&self) -> ([[i8; 3]; 3], f32) {
    (
      [
        [0, -1, 0],
        [-1, 4, -1],
        [0, -1, 0]
      ],
      1.0
    )
  }
}
