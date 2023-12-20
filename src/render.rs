use std::path::Path;
use raylib::prelude::*;

pub fn display_image(path: &Path) {
  let (mut rl, thread) = raylib::init()
      .size(1080, 720)
      .title("Hello, World")
      .build();

  let mut img = Image::load_image(path.to_str().unwrap()).unwrap();
  // Image::load_image_from_mem(filetype, bytes, size);
  img.resize(500, 500);
  let tex = rl.load_texture_from_image(&thread, &img).unwrap();
  while !rl.window_should_close() {
    let mut d = rl.begin_drawing(&thread);

    d.clear_background(Color::WHITE);
    d.draw_texture(&tex, 30, 30, Color::WHITE);
  }
}