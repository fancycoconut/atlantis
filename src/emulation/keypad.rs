use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Keys {
  Unknown = -1,
  A = 0,
  B = 1,
  Select = 2,
  Start = 3,
  Right = 4,
  Left = 5,
  Up = 6,
  Down = 7,
  R = 8,
  L = 9
}

impl FromStr for Keys {
  type Err = ();

  fn from_str(input: &str) -> Result<Keys, Self::Err> {
    match input {
      "A" => Ok(Keys::A),
      "B" => Ok(Keys::B),
      "Select" => Ok(Keys::Select),
      "Start" => Ok(Keys::Start),
      "Right" => Ok(Keys::Right),
      "Left" => Ok(Keys::Left),
      "Up" => Ok(Keys::Up),
      "Down" => Ok(Keys::Down),
      "R" => Ok(Keys::R),
      "L" => Ok(Keys::L),
      _ => Ok(Keys::Unknown)
    }
  }
}
