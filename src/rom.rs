use std::str::from_utf8;
use crate::file_util::read_file;
use crate::file_util::file_exists;
use crate::file_util::get_file_extension;

pub struct ROM {
  pub game_title: String,
  pub game_code: String,
  pub maker_code: String,
  pub main_unit_code: u8,
  pub device_type: u8,
  pub software_version: u8,
  pub checksum: u8,

  pub data: Vec::<u8>
}

impl ROM {
  pub fn new() -> Self {
    ROM {
      game_title: "".to_string(),
      game_code: "".to_string(),
      maker_code: "".to_string(),
      main_unit_code: 0,
      device_type: 0,
      software_version: 0,
      checksum: 0,

      data: Vec::<u8>::new()
    }
  }

  pub fn load(&mut self, file: &str) -> Result<bool, String> {
    if !self.is_supported(file) {
      return Err("An unsupported file type has been loaded".to_string());
    }
    if !file_exists(file) {
      return Err("The loaded file does not exist".to_string());
    }

    self.data = read_file(file);
    _ = self.read_rom_header();

    Ok(true)
  }

  fn read_rom_header(&mut self) -> Result<bool, String> {
    if self.data.len() < 0xC0 {
      return Err("Failed to load ROM header due to file size smaller than header size".to_string());
    }

    self.game_title = from_utf8(&self.data[0xA0..0xAC]).unwrap().to_string();
    self.game_code = from_utf8(&self.data[0xAC..0xB0]).unwrap().to_string();
    self.maker_code = from_utf8(&self.data[0xB0..0xB2]).unwrap().to_string();
    self.main_unit_code = self.data[0xB3];
    self.device_type = self.data[0xB4];
    self.software_version = self.data[0xBC];
    self.checksum = self.data[0xBD];

    Ok(true)
  }

  fn is_supported(&self, file: &str) -> bool {
    get_file_extension(file) == Some("gba") || get_file_extension(file) == Some("agb")
  }
}