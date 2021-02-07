use qk_livesystem::ui::point::Pointf;

/// Editor pointer mode, whether the pointing device (mouse) is busy doing something like dragging
#[derive(Copy, Clone)]
pub enum QkPointerMode {
  Normal,
  /// Mouse pan mode activated, contains starting point
  Pan(Pointf),
}

impl Default for QkPointerMode {
  fn default() -> Self { QkPointerMode::Normal }
}
