pub struct FilteringImage {
  pub width : i32,
  pub height: i32,
  pub bytes: Vec<u8>,
}

impl FilteringImage {
  pub fn copy_from(&mut self, bytes: &[u8]) {
    self.bytes.copy_from_slice(bytes);
  }

  pub fn new(width: u32, height: u32) -> Self {
    Self {
      width: width as i32,
      height: height as i32,
      bytes: vec![0; (width * height * 3) as usize],
    }
  }
}
  