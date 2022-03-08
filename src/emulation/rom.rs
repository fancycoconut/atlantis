use std::str::from_utf8;
use crate::file_util::read_file;
use crate::file_util::get_file_extension;

struct ROM {
  game_title: String,
  game_code: String,
  maker_code: String,
  main_unit_code: u8,
  device_type: u8,
  version: u8,
  checksum: u8,

  data: Vec<u8>
}

impl ROM {
  pub fn load(&self, file: String) -> Result<bool> {
    if !self.is_supported(file) {
      Result::Err(false)
    }

    self.data = read_file(file);
    _ = self.read_rom_header();

    Ok(true)
  }

  fn read_rom_header(&self) -> Result<String> {
    if self.data.len() < 0xC0 {
      Err("Failed to load ROM header due to file size smaller than header size")
    }

    self.game_title = from_utf8(&self.data[0xA0..0xAC]).unwrap();
    self.game_code = from_utf8(&self.data[0xAC..0xB0]).unwrap();
  }

  fn is_supported(&self, file: String) -> bool {
    get_file_extension(&file) == Some("gba") || get_file_extension(&file) == Some("agb")
  }
}