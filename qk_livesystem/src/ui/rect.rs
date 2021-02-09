use std::fmt;
use std::fmt::Debug;
use crate::ui::size::Sizef;
use crate::ui::point::Pointf;

#[derive(Copy, Clone)]
pub struct Rectf {
  pub start: Pointf,
  pub end: Pointf,
}

impl Default for Rectf {
  fn default() -> Self {
    Rectf::new(Pointf::default(), Pointf::default())
  }
}

impl From<[f32; 4]> for Rectf {
  fn from(p: [f32; 4]) -> Self {
    Self::new(
      Pointf::new(p[0], p[1]),
      Pointf::new(p[2], p[3]),
    )
  }
}

impl Into<[f32; 4]> for Rectf {
  fn into(self) -> [f32; 4] {
    [self.start.x, self.start.y,
      self.end.x, self.end.y]
  }
}

impl Debug for Rectf {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Rect({:?}; {:?})", self.start, self.end)
  }
}

impl Rectf {
  pub fn new(start: Pointf, end: Pointf) -> Self {
    Self { start, end }
  }

  pub fn contains_point(&self, point: &Pointf) -> bool {
    println!("Point test {:?} with {:?}", point, self);
    self.start.x <= point.x && self.start.y <= point.y
        && self.end.x >= point.x && self.end.y >= point.y
  }
}

impl std::ops::Add<Sizef> for Rectf {
  type Output = Self;
  fn add(self, other: Sizef) -> Self {
    Self {
      start: self.start + other,
      end: self.end + other,
    }
  }
}
