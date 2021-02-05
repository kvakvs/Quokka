use std::fmt;
use std::fmt::Debug;

#[derive(Copy, Clone)]
pub struct Sizef {
  pub x: f64,
  pub y: f64,
}

impl Debug for Sizef {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Sz({};{})", self.x, self.y)
  }
}

impl Sizef {
  pub fn new(x: f64, y: f64) -> Self {
    Sizef { x, y }
  }
}
