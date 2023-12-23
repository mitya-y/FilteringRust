pub struct FilteringImage<'a> {
  pub height: i32,
  pub width : i32,
  pub bytes: &'a [u8],
}
  
impl<'a> FilteringImage<'a> {
  pub fn new(bytes: &'a [u8], width: u32, height: u32) -> Self {
    Self {
      width: width as i32,
      height: height as i32,
      bytes: bytes
    }
  }
}
  
  pub struct FilteringImageMut<'a> {
    pub width : i32,
    pub _height: i32,
    pub bytes: &'a mut [u8]
  }
  
  impl<'a> FilteringImageMut<'a> {
    pub fn new(bytes: &'a mut [u8], width: u32, height: u32) -> Self {
      Self {
        width: width as i32,
        _height: height as i32,
        bytes: bytes
      }
    }
  }
  