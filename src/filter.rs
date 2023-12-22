use std::path::Path;
use image::{io::Reader as ImageReader, Rgb, ImageBuffer, ImageFormat};

struct FilteringImage<'a> {
  width : i32,
  height: i32,
  bytes: &'a [u8]
}

impl<'a> FilteringImage<'a> {
  fn new(bytes: &'a [u8], width: u32, height: u32) -> Self {
    Self {
      width: width as i32,
      height: height as i32,
      bytes: bytes
    }
  }
}

struct FilteringImageMut<'a> {
  width : i32,
  _height: i32,
  bytes: &'a mut [u8]
}

impl<'a> FilteringImageMut<'a> {
  fn new(bytes: &'a mut [u8], width: u32, height: u32) -> Self {
    Self {
      width: width as i32,
      _height: height as i32,
      bytes: bytes
    }
  }
}

fn get_pixel(img: &FilteringImage, mut x: i32, mut y: i32) -> [u8; 3] {
  x = (x + img.width) % img.width;
  y = (y + img.height) % img.height;
  let first_index = ((y * img.width + x) * 3) as usize;
  [
    img.bytes[first_index + 0],
    img.bytes[first_index + 1],
    img.bytes[first_index + 2],
  ]
}

fn set_pixel(img: &mut FilteringImageMut, x: i32, y: i32, color: [u8; 3]) {
  let first_index = ((y * img.width + x) * 3) as usize;
  img.bytes[first_index + 0] = color[0];
  img.bytes[first_index + 1] = color[1];
  img.bytes[first_index + 2] = color[2];
}

fn calculate_pixel<FilterType : super::FilterMatrix>(img: &FilteringImage, x: i32, y: i32, filter: &FilterType) -> [u8; 3] {  
  let (matrix, coef) = filter.get3x3();

  let mut pixel = [0u8; 3];
  for k in 0..3 {
    let mut sum = 0.0;
    for i in -1..=1 {
      for j in -1..=1 {
        sum += get_pixel(img, x + i, y + j)[k] as f32 * matrix[(i + 1) as usize][(j + 1) as usize] as f32;
      }
    }
    pixel[k] = (sum * coef) as u8;
  }
  pixel
}

fn apply_filter<FilterType : super::FilterMatrix>(source: &FilteringImage, dest: &mut FilteringImageMut, filter: FilterType) {
  for x in 0..source.width {
    for y in 0..source.height {
      set_pixel(dest, x, y, calculate_pixel(source, x, y, &filter));
    }
  }
}

pub fn filter<FilterType : super::FilterMatrix>(source: &Path, dest: &Path, filter: FilterType) {
  let src_img = ImageReader::open(source).unwrap().decode().unwrap();
  let mut bytes_vector = vec![0u8; src_img.width() as usize * src_img.height() as usize * 3];
  
  let source_image = FilteringImage::new(src_img.as_bytes(), src_img.width(), src_img.height());
  let mut dest_image = FilteringImageMut::new(bytes_vector.as_mut_slice(), src_img.width(), src_img.height());

  apply_filter(&source_image, &mut dest_image, filter);
  
  let buf: ImageBuffer<Rgb<u8>, _> = 
    ImageBuffer::from_raw(src_img.width(), src_img.height(), bytes_vector.as_slice()).unwrap();
  buf.save_with_format(dest, ImageFormat::Png).unwrap();

  super::display_images(source, dest);
}
