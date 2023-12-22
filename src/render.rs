use std::path::Path;
use raylib::prelude::*;

pub fn display_images(source: &Path, dest: &Path) {
  let (mut rl, thread) = raylib::init()
      .size(1080, 720)
      .title("Hello, World")
      .build();

  let mut img_sourse = Image::load_image(source.to_str().unwrap()).unwrap();
  let mut img_dest = Image::load_image(dest.to_str().unwrap()).unwrap();
  img_sourse.resize(500, 500);
  img_dest.resize(500, 500);
  let tex_source = rl.load_texture_from_image(&thread, &img_sourse).unwrap();
  let tex_dest = rl.load_texture_from_image(&thread, &img_dest).unwrap();
  while !rl.window_should_close() {
    let mut d = rl.begin_drawing(&thread);

    d.clear_background(Color::WHITE);
    d.draw_texture(&tex_source, 20, 30, Color::WHITE);
    d.draw_texture(&tex_dest, 560, 30, Color::WHITE);
  }
}