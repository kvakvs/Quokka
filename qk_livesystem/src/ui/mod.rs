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

/// Component for nodes and processes, which holds document space positioning, size, color, etc.
pub struct Layout {
  // Document space positioning
  pub pos: Pointf,

  // Document space size, if overridden, otherwise default size box is rendered
  pub size: Option<Sizef>,
}

impl Debug for Layout {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Layout(@{:?}, {:?})", self.pos, self.size)
  }
}

impl Layout {
  pub fn new(pos: Pointf) -> Self {
    Layout { pos, size: None }
  }
}

pub trait TLayout {
  fn layout_pos(&self) -> &Pointf;
  fn layout_pos_mut(&mut self) -> &mut Pointf;
  fn layout_size(&self) -> &Option<Sizef>;
  fn layout_size_mut(&mut self) -> &mut Option<Sizef>;
}
