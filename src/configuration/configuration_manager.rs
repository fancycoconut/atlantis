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
    let data = fs::read_to_string(&self.config_file)
      .expect("Unable to read config file");

    serde_json::from_str(&data).unwrap()
  }

}