pub struct NegativeFilter;

impl super::FilterMatrix for NegativeFilter {
  fn get3x3(&self) -> ([[u8; 3]; 3], f32) {
    (
      [
        [0, 0, 0],
        [0, 0, 0],
        [0, 0, 0]
      ],
      1.0
    )
  }
}
