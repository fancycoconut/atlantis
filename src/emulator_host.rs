//use crate::rom::ROM;
use crate::app_constants as AppConstants;
use crate::engine::window::Window;
use crate::engine::sdl::sdl_window::SDLWindow;
use crate::emulation::gba::Gba;
use crate::emulation::rom::ROM;
use crate::emulation::gba_constants as Constants;
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
      window: SDLWindow::new(Constants::GBA_WIDTH, Constants::GBA_HEIGHT, zoom)
    }
  }

  pub fn set_rom(&self, file: String) {
    let mut rom = ROM::new();
    rom.load(&file);
  }

  pub fn start(&mut self) {
    self.window.show(AppConstants::TITLE.to_string());

    self.running = true;
    self.main_emulation_loop();
  }

  fn main_emulation_loop(&self) {
    while self.running {
      println!("Emulator running");
    }
  }
}
