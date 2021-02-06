use crate::ui::ui_element_state::UiElementState;

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

pub const SELECTED_COLOR: StyleColor = StyleColor::RGB(0.2, 0.4, 0.9);

pub const FONT_NORMAL_COLOR: StyleColor = StyleColor::RGB(0.0, 0.0, 0.0);
pub const FONT_SELECTED_COLOR: StyleColor = SELECTED_COLOR;

pub fn font_color(st: UiElementState) -> StyleColor {
  match st {
    UiElementState::NotSelected => { FONT_NORMAL_COLOR }
    UiElementState::Selected => { FONT_SELECTED_COLOR }
  }
}

pub const FONT_SELECTED_BACKGROUND: StyleColor = StyleColor::RGB(0.7, 0.8, 0.9);

pub fn font_background(st: UiElementState) -> StyleColor {
  match st {
    UiElementState::NotSelected => { StyleColor::None }
    UiElementState::Selected => { FONT_SELECTED_BACKGROUND }
  }
}

pub const BACKGROUND_COLOR: StyleColor = StyleColor::RGB(1.0, 1.0, 1.0);

pub const LINE_NORMAL_COLOR: StyleColor = StyleColor::RGB(0.2, 0.2, 0.3);
pub const LINE_SELECTED_COLOR: StyleColor = SELECTED_COLOR;

pub fn line_color(st: UiElementState) -> StyleColor {
  match st {
    UiElementState::NotSelected => { LINE_NORMAL_COLOR }
    UiElementState::Selected => { LINE_SELECTED_COLOR }
  }
}

pub const FONT_SIZE: f64 = 12.0;
pub const SELECTED_LINE_WIDTH: f64 = 3.0;
pub const LINE_WIDTH: f64 = 1.0;