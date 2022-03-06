use serde::{Deserialize};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct EmulatorConfiguration {
  emulator: EmulatorSettings,
  controller: ControllerSettings,
}

#[derive(Deserialize)]
struct EmulatorSettings {
  zoom: i32,
}

#[derive(Deserialize)]
//#[allow(non_snake_case)]
struct ControllerSettings {
  key_mappings: HashMap<String, String>,
}