use sdl2;
use crate::engine::window::Window as Window;

pub struct SDLWindow {
  width: i32,
  height: i32,
  zoom: i32,
}

impl SDLWindow {
  pub fn new(width: i32, height: i32, zoom: i32) -> SDLWindow {
    SDLWindow {
      width: width,
      height: height,
      zoom: zoom
    }
  }
}

impl Window for SDLWindow {
  fn show(&self, title: String) {
    let target_width = (&self.width * &self.zoom) as u32;
    let target_height = (&self.height * &self.zoom) as u32;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
      .window(&title, target_width, target_height)
      .position_centered()
      .build();
  }

  fn set_title(&self, title: String) {

  }

  fn draw(&self) {

  }
}