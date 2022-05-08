use crate::rom::ROM;

pub struct Gba {
  rom: ROM,
}

impl Gba {
  pub fn new(rom: ROM) -> Self {
    Gba {
      rom: rom
    }
  }

  pub fn start() {

  }
}