mod engine;
mod app_constants;
mod emulation;
mod emulator_host;
mod configuration;

use emulator_host::EmulatorHost as EmulatorHost;
use configuration::configuration_manager::ConfigurationManager as ConfigurationManager;

fn main() {
  println!("Hello World!");

  let config = ConfigurationManager::new("appsettings.json".to_string())
    .build();

  let emulator = EmulatorHost::new(config);
  emulator.test();
  emulator.start();

}