pub struct Size<T> {
  pub x: T,
  pub y: T,
}

impl<T> Size<T> {
  pub fn new(x: T, y: T) -> Self {
    Size { x, y }
  }
}

pub type Sizei32 = Size<i32>;
