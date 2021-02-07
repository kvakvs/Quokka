use std::fmt;
use std::fmt::Debug;

#[derive(Copy, Clone)]
pub struct Pointf {
  pub x: f64,
  pub y: f64,
}

impl From<(f64, f64)> for Pointf {
  fn from(p: (f64, f64)) -> Self {
    Self::new(p.0, p.1)
  }
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

impl std::ops::Add for Pointf {
  type Output = Self;
  fn add(self, other: Self) -> Self {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

impl std::ops::Sub for Pointf {
  type Output = Self;
  fn sub(self, other: Self) -> Self {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
    }
  }
}
