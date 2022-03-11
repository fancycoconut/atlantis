use sdl2::event::Event;
use crate::engine::events::Events;
use crate::engine::events::EventsCallback;

pub struct SDLEvents {
  exit_handler: EventsCallback,
  keypress_handler: EventsCallback,
}

impl SDLEvents {
  pub fn new (exit_handler: EventsCallback, keypress_handler: EventsCallback) -> Self {
    Self { 
      exit_handler: exit_handler,
      keypress_handler: keypress_handler
    }
  }
}

impl Events for SDLEvents {
  fn poll(&self) {
    for event in event_pump
  }
}

