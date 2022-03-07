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
    let target_width = &self.width * &self.zoom;
    let target_height = &self.height * &self.zoom;
  }

  fn set_title(&self, title: String) {

  }

  fn draw(&self) {

  }
}