use std::include_bytes;
use sdl2::event::Event;

// mod file_util;
// mod engine;
// mod app_constants;
// mod emulation;
// mod emulator;
mod configuration;
mod constants;

use configuration::configuration_manager::ConfigurationManager;

fn main() {
  println!("Hello World!");

  let bios = include_bytes!("assets/GBA.BIOS");
  let config = ConfigurationManager::new("appsettings.json".to_string())
    .build();

  let sdl_context = sdl2::init().expect("Failed to initialize SDL2");
  let video_subsystem = sdl_context.video().unwrap();

  let target_width = (constants::GBA_WIDTH * config.emulator.zoom) as u32;
  let target_height = (constants::GBA_HEIGHT * config.emulator.zoom) as u32;

  let mut window = match video_subsystem
    .window(constants::TITLE, target_width, target_height)
    .opengl()
    .position_centered()
    .resizable()
    .build() {
      Ok(window) => window,
      _ => panic!("An error ocurred while initializing an SDL window")
    };

  let mut running = true;
  let mut event_pump = sdl_context.event_pump().unwrap();
  while running {
    handle_events(&mut running, &mut event_pump);
  }
}

fn handle_events(emulator_is_running: &mut bool, event_pump: &mut sdl2::EventPump) {
  for event in event_pump.poll_iter() {
    match event {
      Event::Quit { .. } => {
        *emulator_is_running = false;
        println!("Emulator stopped running... Exiting...");
      },
      Event::KeyDown { .. } => {
        
      },
      _ => {
        
      }
    }
  }
}