use std::path::Path;
use image::io::Reader as ImageReader;

pub fn filter<FilterType : super::FilterMatrix>(source: &Path, _dest: &Path, filter: FilterType) {
  let img = ImageReader::open(source).unwrap().decode().unwrap();
  let _bytes = img.as_bytes();
  let matrix = filter.get3x3();

  super::display_image(source);
}
