
pub trait Window {
  fn show(&self, title: String);
  fn set_title(&self, title: String);
  fn draw(&self);  
}