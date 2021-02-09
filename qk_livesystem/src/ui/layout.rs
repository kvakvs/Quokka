use std::fmt;
use std::fmt::Debug;

use crate::ui::point::Pointf;
use crate::ui::size::Sizef;
use crate::ui::rect::Rectf;

/// Component for nodes and processes, which holds document space positioning, size, color, etc.
pub struct Layout {
  /// Document space positioning
  pub center_pos: Pointf,

  /// Document space size, if overridden, otherwise default size box is rendered
  pub size: Sizef,

  pub draw_box: Rectf,
}

impl Debug for Layout {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Layout(@{:?}, {:?})", self.center_pos, self.size)
  }
}

impl Layout {
  pub fn new(pos: Pointf) -> Self {
    Layout {
      center_pos: pos,
      size: Self::size_or_default(None),
      draw_box: Default::default(),
    }.update_draw_box()
  }

  fn size_or_default(sz: Option<Sizef>) -> Sizef {
    sz.unwrap_or_else(|| Sizef::new(20.0, 20.0))
  }

  fn update_draw_box(mut self) -> Self {
    let origin = Pointf::new(
      self.center_pos.x - 0.5 * self.size.x,
      self.center_pos.y - 0.5 * self.size.y,
    );

    self.draw_box = Rectf::new(origin, origin + self.size);
    self
  }
}

// pub trait TLayout {
//   fn layout_pos(&self) -> &Pointf;
//   fn layout_pos_mut(&mut self) -> &mut Pointf;
//   fn layout_size(&self) -> &Option<Sizef>;
//   fn layout_size_mut(&mut self) -> &mut Option<Sizef>;
// }
