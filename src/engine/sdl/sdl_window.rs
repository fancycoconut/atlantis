use sdl2;

struct SDLWindow {
  width: i32,
  height: i32,
  zoom: i32,

}

impl Window for SDLWindow {
  pub fn new(width: i32, height: i32, zoom: i32) -> SDLWindow {
    SDLWindow {
      width: width,
      height: height,
      zoom: zoom
    }
  }

  pub fn show(&self, title: String) {
    let target_width = width * zoom;
    let target_height = height * height;
  }

  pub fn set_title(&self, title: String) {

  }

  pub fn draw(&self) {

  }
}