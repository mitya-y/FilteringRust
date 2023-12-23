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

pub fn get_pixel(&self, mut x: i32, mut y: i32) -> [u8; 3] {
  x = (x + self.width) % self.width;
  y = (y + self.height) % self.height;
  let first_index = ((y * self.width + x) * 3) as usize;
  [
    self.bytes[first_index + 0],
    self.bytes[first_index + 1],
    self.bytes[first_index + 2],
  ]
}

pub fn set_pixel(&mut self, x: i32, y: i32, color: [u8; 3]) {
  let first_index = ((y * self.width + x) * 3) as usize;
  self.bytes[first_index + 0] = color[0];
  self.bytes[first_index + 1] = color[1];
  self.bytes[first_index + 2] = color[2];
}
}
  