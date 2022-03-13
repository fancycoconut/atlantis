use sdl2;
use sdl2::Sdl;
use sdl2::EventPump;
use sdl2::video::Window;
use sdl2::VideoSubsystem;
use crate::engine::window::Window as IWindow;
use crate::engine::events::Events;

pub struct SDLWindow {
  sdl_context: Sdl,
  video_subsystem: VideoSubsystem,
  event_pump: EventPump,

  width: i32,
  height: i32,
  zoom: i32,
  window: Option<Window>
}

impl SDLWindow {
  pub fn new(width: i32, height: i32, zoom: i32) -> SDLWindow {
    let sdl_context = sdl2::init().expect("Failed to initialize SDL2");
    let video_subsystem = sdl_context.video().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    SDLWindow {
      sdl_context: sdl_context,
      video_subsystem: video_subsystem,
      event_pump: event_pump,

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

    let window = match self.video_subsystem
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

impl Events for SDLWindow {
  fn poll(&self) {
    
  }
}
