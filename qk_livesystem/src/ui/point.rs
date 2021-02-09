use std::fmt;
use std::fmt::Debug;
use crate::ui::size::Sizef;

#[derive(Copy, Clone)]
pub struct Pointf {
  pub x: f32,
  pub y: f32,
}

impl Default for Pointf {
  fn default() -> Self {
    Pointf::new(0.0, 0.0)
  }
}

impl From<[f32; 2]> for Pointf {
  fn from(p: [f32; 2]) -> Self {
    Self::new(p[0], p[1])
  }
}

impl Into<[f32; 2]> for Pointf {
  fn into(self) -> [f32; 2] {
    [self.x, self.y]
  }
}

impl Debug for Pointf {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Pos({};{})", self.x, self.y)
  }
}

impl Pointf {
  pub fn new(x: f32, y: f32) -> Self {
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

impl std::ops::Add<Sizef> for Pointf {
  type Output = Self;
  fn add(self, other: Sizef) -> Self {
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
