
struct ROM {
  game_title: String,
  game_code: String,
  maker_code: String,
  main_unit_code: u8,
  device_type: u8,
  version: u8,
  checksum: u8,
}

impl ROM {
  pub fn load(&self, file: String) {

  }
}