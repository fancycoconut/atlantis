use std::vec::Vec;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::ffi::OsStr;

pub fn read_file(file: String) -> Vec<u8> {
  // https://stackoverflow.com/questions/31192956/whats-the-de-facto-way-of-reading-and-writing-files-in-rust-1-x
  let mut buffer = Vec::new();
  let mut file = match File::open(&file) {
    Ok(file) => file,
    _ => panic!("Unable open {}", &file)
  };

  let _ = file.read_to_end(&mut buffer);
  buffer
}

pub fn get_file_extension(file: &str) -> Option<&str> {
  Path::new(file)
    .extension()
    .and_then(OsStr::to_str)
}