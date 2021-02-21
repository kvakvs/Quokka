use rand::Rng;
use std::fmt;
use std::fmt::Debug;
use crate::ui::size::Sizef;

pub union FastSqrt {
  f: f32,
  i: i32,
}

impl FastSqrt {
  /// Quake's fast sqrt
  pub fn inv_sqrt(x: f32) -> f32 {
    let mut u = Self { f: x };
    unsafe {
      u.i = 0x5f3759df - (u.i >> 1);
      u.f * (1.5 - 0.5 * x * u.f * u.f)
    }
  }
}

/// Represents a point on 2d plane, and also a vector
#[derive(Copy, Clone)]
pub struct Pointf {
  pub x: f32,
  pub y: f32,
}

impl Pointf {
  pub fn normalize(&self) -> Self {
    let m = self.magnitude();
    if m.abs() < 0.00001 { return Self { x: 0.0, y: 0.0 }; }

    let inv_m = 1.0 / m;
    Self {
      x: self.x * inv_m,
      y: self.y * inv_m,
    }
  }

  pub fn magnitude(&self) -> f32 {
    FastSqrt::inv_sqrt(self.squared_magnitude())
  }

  pub fn squared_magnitude(&self) -> f32 {
    self.x * self.x + self.y * self.y
  }
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
  pub fn new_random() -> Self {
    let mut rng = rand::thread_rng();
    Pointf {
      x: rng.gen::<f32>() * 800.0,
      y: rng.gen::<f32>() * 500.0,
    }
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

impl std::ops::AddAssign<Pointf> for Pointf {
  fn add_assign(&mut self, other: Self) {
    self.x += other.x;
    self.y += other.y;
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

impl std::ops::Div<f32> for Pointf {
  type Output = Self;
  fn div(self, scale: f32) -> Self {
    let inv = 1.0 / scale;
    Self {
      x: self.x * scale,
      y: self.y * scale,
    }
  }
}

impl std::ops::Mul<f32> for Pointf {
  type Output = Self;
  fn mul(self, scale: f32) -> Self {
    Self {
      x: self.x * scale,
      y: self.y * scale,
    }
  }
}

impl std::ops::MulAssign<f32> for Pointf {
  fn mul_assign(&mut self, scale: f32) {
    self.x *= scale;
    self.y *= scale;
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
