
mod emulator_host;

fn main() {
  println!("Hello World!");

  let emulator = emulator_host::EmulatorHost::new();
  emulator.test();

}