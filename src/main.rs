use std::env;
use std::include_bytes;
use std::collections::HashMap;
use std::str::FromStr;
use sdl2::event::Event;

mod configuration;
mod constants;
mod emulation;
mod file_util;
mod rom;

use configuration::configuration_manager::ConfigurationManager;
use emulation::keypad::Keys;
use rom::ROM;

fn main() {
  println!("Hello World!");
  let args: Vec<_> = env::args().collect();

  let bios = include_bytes!("assets/GBA.BIOS");
  let config = ConfigurationManager::new("appsettings.json".to_string())
    .build();

  let sdl_context = sdl2::init().expect("Failed to initialize SDL2");
  let video_subsystem = sdl_context.video().unwrap();

  let target_width = (constants::GBA_WIDTH * config.emulator.zoom) as u32;
  let target_height = (constants::GBA_HEIGHT * config.emulator.zoom) as u32;

  let window = match video_subsystem
    .window(constants::TITLE, target_width, target_height)
    .opengl()
    .position_centered()
    .resizable()
    .build() {
      Ok(window) => window,
      _ => panic!("An error ocurred while initializing an SDL window")
    };

  let mut rom_path = "".to_string();
  if args.len() > 1 {
    rom_path = args[1].clone();
    println!("Loaded ROM from command-line arg {}", rom_path);
  }

  let mut rom = ROM::new();
  // TODO Handle errors and report to user
  _ = rom.load(&rom_path);

  let mut running = true;
  let mut event_pump = sdl_context.event_pump().unwrap();
  while running {
    handle_events(&mut running,
      &mut rom_path,
      &mut rom,
      &config.controller.key_mappings, 
      &mut event_pump);
  }
}

fn handle_events(emulator_is_running: &mut bool, 
  rom_path: &mut String,
  rom: &mut ROM,
  key_mappings: &HashMap<String, String>,
  event_pump: &mut sdl2::EventPump) {
  for event in event_pump.poll_iter() {
    match event {
      Event::Quit { .. } => {
        *emulator_is_running = false;
        println!("Emulator stopped running... Exiting...");
      },
      Event::KeyDown { 
        keycode: Some(keycode), 
        .. 
      } => {
        let key_mapping = match key_mappings.get(&keycode.to_string()) {
          Some(key) => key,
          _ => ""
        };

        let key = Keys::from_str(key_mapping).unwrap();
        println!("Keypress detected {:?}", key);
      },
      Event::DropFile { filename, .. } => {
        *rom_path = filename;
        _ = rom.load(&rom_path);
        println!("Loaded ROM from `{}`", rom_path);
      },
      _ => {
        // Ignore everything else
      }
    }
  }
}