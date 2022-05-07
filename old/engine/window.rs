
pub trait Window {
  fn show(&mut self, title: String);
  fn set_title(&mut self, title: String);
  fn draw(&self);
}