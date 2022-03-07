//use crate::rom::ROM as ROM;
use crate::app_constants;
use crate::engine::window::Window;
use crate::engine::sdl::sdl_window::SDLWindow as SDLWindow;
use crate::emulation::gba_constants;
use crate::configuration::emulator_configuration::EmulatorConfiguration as EmulatorConfiguration;

pub struct EmulatorHost {
  config :EmulatorConfiguration,
  window: SDLWindow,
  running: bool,
}

impl EmulatorHost {
  pub fn new(config :EmulatorConfiguration) -> EmulatorHost {
    let zoom = config.emulator.zoom;

    EmulatorHost {
      config: config,
      running: false,
      window: SDLWindow::new(gba_constants::GBA_WIDTH, gba_constants::GBA_HEIGHT, zoom)
    }
  }

  pub fn start(&self) {
    self.window.show(app_constants::TITLE.to_string());
  }

  pub fn test(&self) {
    println!("It works!");
  }
}
