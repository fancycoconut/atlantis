use std::fs;
use crate::configuration::emulator_configuration::EmulatorConfiguration;

pub struct ConfigurationManager {
  config_file: String,
}

impl ConfigurationManager {
  pub fn new(config_file: String) -> ConfigurationManager {
    ConfigurationManager {
      config_file: config_file
    }
  }

  pub fn build(&self) -> EmulatorConfiguration {
    let data = fs::read_to_string(&self.config_file);

    let data = match data {
      Ok(text) => text,
      _ => panic!("Unable to read {}", self.config_file)
    };

    let emulator_configuration = match serde_json::from_str(&data) {
      Ok(config) => config,
      _ => panic!("Unable to parse application config")
    };

    emulator_configuration
  }

}