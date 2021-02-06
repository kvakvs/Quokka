use crate::ui::styling::DEFAULT_SELECTED_COLOR;
use crate::ui::styling::style_color::StyleColor;
use crate::ui::ui_element_state::UiElementState;

pub const SELECTED_LINE_WIDTH: f64 = 3.0;
pub const LINE_WIDTH: f64 = 1.0;

pub const LINE_NORMAL_COLOR: StyleColor = StyleColor::RGB(0.2, 0.2, 0.3);
pub const LINE_SELECTED_COLOR: StyleColor = DEFAULT_SELECTED_COLOR;

pub fn line_color(st: UiElementState) -> StyleColor {
  match st {
    UiElementState::NotSelected => { LINE_NORMAL_COLOR }
    UiElementState::Selected => { LINE_SELECTED_COLOR }
  }
}

/// Applies line color width etc. according to the UI element state
pub fn apply(cr: &cairo::Context, ui_element_state: UiElementState) {
  match ui_element_state {
    UiElementState::NotSelected => {
      cr.set_line_width(LINE_WIDTH);
      LINE_NORMAL_COLOR.set_source_rgb(cr);
    }
    UiElementState::Selected => {
      cr.set_line_width(SELECTED_LINE_WIDTH);
      LINE_SELECTED_COLOR.set_source_rgb(cr);
    }
  }
}
