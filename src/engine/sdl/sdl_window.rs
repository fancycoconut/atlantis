use sdl2;
use sdl2::video::Window;
use crate::engine::window::Window as IWindow;



pub struct SDLWindow {
  width: i32,
  height: i32,
  zoom: i32,
  window: Option<Window>
}

impl SDLWindow {
  pub fn new(width: i32, height: i32, zoom: i32) -> SDLWindow {
    SDLWindow {
      width: width,
      height: height,
      zoom: zoom,
      window: Option::None
    }
  }
}

impl IWindow for SDLWindow {
  fn show(&mut self, title: String) {
    let target_width = (&self.width * &self.zoom) as u32;
    let target_height = (&self.height * &self.zoom) as u32;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = match video_subsystem
      .window(&title, target_width, target_height)
      .position_centered()
      .resizable()
      .build() {
        Ok(window) => window,
        _ => panic!("An error ocurred while initializing an SDL window")
      };

    self.window = Option::Some(window);
  }

  fn set_title(&mut self, title: String) {
    let window = self.window.as_mut().unwrap();
    _ = window.set_title(&title);
  }

  fn draw(&self) {
  }
}