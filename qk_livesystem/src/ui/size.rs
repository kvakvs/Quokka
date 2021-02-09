use std::fmt;
use std::fmt::Debug;

#[derive(Copy, Clone)]
pub struct Sizef {
  pub x: f32,
  pub y: f32,
}

impl Debug for Sizef {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Sz({};{})", self.x, self.y)
  }
}

impl Sizef {
  pub fn new(x: f32, y: f32) -> Self {
    Sizef { x, y }
  }
}
