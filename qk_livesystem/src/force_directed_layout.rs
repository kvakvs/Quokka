use crate::ui::layout::Layout;

pub struct Force {
  x: f32,
  y: f32,
}

impl Force {
  pub fn new() -> Self {
    Force {
      x: 0.0,
      y: 0.0,
    }
  }

  /// Add a force between point and other point, using weight. Positive for repulsion, negative for
  /// attraction.
  pub fn add_force(&mut self, point: &Layout, other_point: &Layout, weight: f32) {}
}
