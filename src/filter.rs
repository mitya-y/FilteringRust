use std::path::Path;
use image::{io::Reader as ImageReader, Rgb, ImageBuffer, ImageFormat};
use super::FilteringImage;

fn calculate_pixel<FilterType : super::FilterMatrix>(img: &FilteringImage, x: i32, y: i32, filter: &FilterType) -> [u8; 3] {  
  let (matrix, coef) = filter.get3x3();

  let mut pixel = [0u8; 3];
  for k in 0..3 {
    let mut sum = 0.0;
    for i in -1..=1 {
      for j in -1..=1 {
        sum += img.get_pixel(x + i, y + j)[k] as f32 * matrix[(i + 1) as usize][(j + 1) as usize] as f32;
      }
    }
    pixel[k] = (sum * coef) as u8;
  }
  pixel
}

fn apply_filter<FilterType : super::FilterMatrix>(source: &FilteringImage, dest: &mut FilteringImage, filter: &FilterType) {
  for x in 0..source.width {
    for y in 0..source.height {
      dest.set_pixel(x, y, calculate_pixel(source, x, y, filter));
    }
  }
}

pub fn filter<FilterType : super::FilterMatrix>(source: &Path, dest: &Path, filters: Vec<FilterType>) {
  let src_img = ImageReader::open(source).unwrap().decode().unwrap();
  
  let mut source_image = FilteringImage::new(src_img.width(), src_img.height());
  source_image.copy_from(src_img.as_bytes());
  let mut dest_image = FilteringImage::new(src_img.width(), src_img.height());

  apply_filter(&source_image, &mut dest_image, &filters[0]);
  
  let buf: ImageBuffer<Rgb<u8>, _> = 
    ImageBuffer::from_raw(src_img.width(), src_img.height(), dest_image.bytes).unwrap();
  buf.save_with_format(dest, ImageFormat::Png).unwrap();

  super::display_images(source, dest);
}
