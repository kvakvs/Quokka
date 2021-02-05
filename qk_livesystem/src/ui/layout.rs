use std::fmt;
use std::fmt::Debug;

use crate::ui::point::Pointf;
use crate::ui::size::Sizef;

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
