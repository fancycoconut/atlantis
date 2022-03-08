//use crate::rom::ROM;
use crate::app_constants;
use crate::engine::window::Window;
use crate::engine::sdl::sdl_window::SDLWindow;
use crate::emulation::gba::Gba;
use crate::emulation::rom::ROM;
use crate::emulation::gba_constants;
use crate::configuration::emulator_configuration::EmulatorConfiguration;

pub struct EmulatorHost {
  config: EmulatorConfiguration,
  window: SDLWindow,
  running: bool,
  gba: Gba,
}

impl EmulatorHost {
  pub fn new(config :EmulatorConfiguration) -> EmulatorHost {
    let zoom = config.emulator.zoom;

    EmulatorHost {
      config: config,
      running: false,
      gba: Gba::new(),
      window: SDLWindow::new(gba_constants::GBA_WIDTH, gba_constants::GBA_HEIGHT, zoom)
    }
  }

  pub fn set_rom(file: String) {

  }

  pub fn start(&mut self) {
    self.window.show(app_constants::TITLE.to_string());

    self.running = true;
    self.main_emulation_loop();
  }

  fn main_emulation_loop(&self) {
    while self.running {
      println!("Emulator running");
    }
  }
}
