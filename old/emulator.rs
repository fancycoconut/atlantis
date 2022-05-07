use crate::app_constants as AppConstants;
use crate::engine::window::Window;
use crate::engine::sdl::sdl_window::SDLWindow;
use crate::emulation::gba::Gba;
use crate::emulation::rom::ROM;
use crate::emulation::gba_constants as Constants;
use crate::configuration::emulator_configuration::EmulatorConfiguration;

pub struct Emulator {
  window: SDLWindow,
  config: EmulatorConfiguration,

  gba: Gba,
  rom: ROM,
  running: bool,
}

impl Emulator {
  pub fn new(config :EmulatorConfiguration) -> Emulator {
    let zoom = config.emulator.zoom;
    let window = SDLWindow::new(Constants::GBA_WIDTH, Constants::GBA_HEIGHT, zoom);

    Emulator {
      running: false,
      gba: Gba::new(),
      rom: ROM::new(),

      config: config,
      window: window
    }
  }

  pub fn set_rom(&mut self, file: String) {
    let mut rom = ROM::new();
    // TODO Handle errors and report to user
    _ = rom.load(&file);
    self.rom = rom;
  }

  pub fn start(&mut self) {
    if self.rom.data.len() == 0 {
      panic!("No ROM has been loaded!");
    }

    self.window.show(AppConstants::TITLE.to_string());
    self.gba.start(&self.rom);

    self.running = true;
    self.main_emulation_loop();
  }

  fn main_emulation_loop(&self) {
    while self.running {
      println!("Emulator running");
    }
  }

  fn exit_handler(&mut self) {
    self.running = false;
    println!("Emulator exit detected... Shutting down.");
  }

  fn keypress_handler(&mut self) {

  }

}
