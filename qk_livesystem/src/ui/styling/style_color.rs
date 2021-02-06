pub enum StyleColor {
  RGB(f64, f64, f64),
  None,
}

impl StyleColor {
  pub fn set_source_rgb(&self, cr: &cairo::Context) {
    match self {
      StyleColor::RGB(r, g, b) => {
        cr.set_source_rgb(*r, *g, *b);
      }
      StyleColor::None => {}
    }
  }

  /// Fill color rectangle if there was a color in the style
  pub fn fill_rectangle(&self, cr: &cairo::Context, x: f64, y: f64, width: f64, height: f64) {
    match self {
      StyleColor::RGB(_, _, _) => {
        self.set_source_rgb(cr);
        cr.rectangle(x, y, width, height);
        cr.fill();
      }
      StyleColor::None => {}
    }
  }

  pub fn clear_with_color(&self, cr: &cairo::Context) {
    match self {
      StyleColor::RGB(r, g, b) => {
        cr.set_source_rgb(*r, *g, *b);
        cr.paint();
      }
      StyleColor::None => {}
    }
  }
}
