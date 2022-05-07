use std::include_bytes;

mod file_util;
mod engine;
mod app_constants;
mod emulation;
mod emulator;
mod configuration;

use emulator::Emulator;
use configuration::configuration_manager::ConfigurationManager;

fn main() {
  println!("Hello World!");

  let bios = include_bytes!("assets/GBA.BIOS");
  let config = ConfigurationManager::new("appsettings.json".to_string())
    .build();

  let mut emulator = Emulator::new(config);
  //emulator.set_rom("/Users/kawaiwong/Documents/armwrestler-gba-fixed.gba".to_string());
  emulator.set_rom("C:\\Users\\Kawai\\Desktop\\armwrestler-gba-fixed.gba".to_string());
  emulator.start();
}