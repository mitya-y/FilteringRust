pub struct FilteringImage {
  pub width : i32,
  pub height: i32,
  pub bytes: Vec<u8>,
}

impl FilteringImage {
  pub fn copy_from(bytes:  &[u8], width: u32, height: u32) -> Self {
    let mut result: Self;
    result.width = width as i32;
    result.height = height as i32;
    result.bytes = Vec::with_capacity((width * height * 3) as usize);
    result.bytes.copy_from_slice(bytes);
    result
  }

  pub fn new(width: u32, height: u32) -> Self {
    let mut result: Self;
    result.width = width as i32;
    result.height = height as i32;
    result.bytes = vec![0; (width * height * 3) as usize];
    result
  }
}
  