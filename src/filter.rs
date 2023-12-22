use std::path::Path;
use image::{io::Reader as ImageReader, GenericImageView, Rgb, DynamicImage, Rgba, EncodableLayout};

fn get_pixel(img : &DynamicImage, x: i32, y: i32) -> Rgba<u8> {
  img.get_pixel((x + img.width() as i32) as u32 % img.width(),
                (y +img.height() as i32) as u32 % img.height())
}

fn calculate_color(img : &DynamicImage, x: u32, y: u32) -> Rgb<u8> {
  let mut edited_pixel = Rgb::from([0; 3]);

  for k in 0..3 {
    let mut sum = 0.0;
    for i in -1..=1 {
      for j in -1..=1 {
        sum += get_pixel(img, x as i32+ i, y as i32 + j)[k] as f32;
      }
    }
    edited_pixel[k] = (sum / 9.0) as u8;
  }

  edited_pixel
}

pub fn filter<FilterType : super::FilterMatrix>(source: &Path, dest: &Path, filter: FilterType) {
  let img = ImageReader::open(source).unwrap().decode().unwrap();
  
  let width = img.width();
  let height = img.height();

  let mut edited: image::ImageBuffer<Rgb<u8>, Vec<_>> = image::ImageBuffer::new(width, height);
  for x in 0..width {
    for y in 0..height {
      let edited_pixel = edited.get_pixel_mut(x, y);
      *edited_pixel = calculate_color(&img, x, y);
    }
  }
  
  image::save_buffer_with_format(dest, &edited, width, height, 
    image::ColorType::Rgb8, image::ImageFormat::Png).unwrap();
  super::display_images(source, dest);
}
