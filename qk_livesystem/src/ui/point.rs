use std::fmt;
use std::fmt::Debug;

#[derive(Copy, Clone)]
pub struct Pointf {
  pub x: f64,
  pub y: f64,
}

impl Debug for Pointf {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Pos({};{})", self.x, self.y)
  }
}

impl Pointf {
  pub fn new(x: f64, y: f64) -> Self {
    Pointf { x, y }
  }
}
