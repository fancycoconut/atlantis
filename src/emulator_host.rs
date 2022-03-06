//use crate::rom::ROM as ROM;
use crate::engine::sdl::sdl_window::SDLWindow as SDLWindow;
use crate::configuration::emulator_configuration::EmulatorConfiguration as EmulatorConfiguration;

pub struct EmulatorHost {
  config :EmulatorConfiguration,
  window: SDLWindow,
}

impl EmulatorHost {
  pub fn new(config :EmulatorConfiguration) -> EmulatorHost {
    EmulatorHost {
      config: config,
      window: SDLWindow::new()
    }
  }

  pub fn start() {

  }

  pub fn test(&self) {
    println!("It works!");
  }
}
