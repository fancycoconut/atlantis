mod file_util;
mod engine;
mod app_constants;
mod emulation;
mod emulator_host;
mod configuration;

use emulator_host::EmulatorHost;
use configuration::configuration_manager::ConfigurationManager;

fn main() {
  println!("Hello World!");

  let config = ConfigurationManager::new("appsettings.json".to_string())
    .build();

  let mut emulator = EmulatorHost::new(config);
  emulator.set_rom("/Users/kawaiwong/Documents/armwrestler-gba-fixed.gba".to_string());
  emulator.start();
}