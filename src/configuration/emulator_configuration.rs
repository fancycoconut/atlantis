use serde::{Deserialize};
use std::collections::HashMap;

use crate::emulation::keypad::Keys;

#[derive(Deserialize)]
pub struct EmulatorConfiguration {
  pub emulator: EmulatorSettings,
  pub controller: ControllerSettings,
}

#[derive(Deserialize)]
pub struct EmulatorSettings {
  pub zoom: i32,
}

#[derive(Deserialize)]
//#[allow(non_snake_case)]
pub struct ControllerSettings {
  pub key_mappings: HashMap<String, String>,
}