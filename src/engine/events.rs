
// Aliases
pub type EventsCallback = fn();

// Interfaces
pub trait Events {
  fn poll(&self); 
}