use crate::engine::

type Callback = fn();

pub struct SDLEvents {
  exit_handler: Callback,
  keypress_handler: Callback,
}

impl SDLEvents {
  pub fn new (exit_handler: Callback, keypress_handler: Callback,) -> Self {
    Self { 
      exit_handler: exit_handler,
      keypress_handler: keypress_handler
    }
  }
}

impl 

