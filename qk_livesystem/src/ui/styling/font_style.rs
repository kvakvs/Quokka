use crate::ui::styling::DEFAULT_SELECTED_COLOR;
use crate::ui::styling::style_color::StyleColor;
use crate::ui::ui_element_state::UiElementState;

pub const FONT_NORMAL_COLOR: StyleColor = StyleColor::RGB(0.0, 0.0, 0.0);
pub const FONT_SELECTED_COLOR: StyleColor = DEFAULT_SELECTED_COLOR;

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

pub const FONT_SIZE: f64 = 12.0;

/// Applies text color width etc. according to the UI element state
pub fn apply(cr: &cairo::Context, ui_element_state: UiElementState) {
  cr.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Normal);
  font_color(ui_element_state).set_source_rgb(cr);
  cr.set_font_size(FONT_SIZE);
}

/// Applies text settings for information labels, rendered in the screen corner
pub fn apply_info_overlay_font_style(cr: &cairo::Context) {
  cr.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Normal);
  cr.set_font_size(FONT_SIZE);
  FONT_NORMAL_COLOR.set_source_rgb(cr);
}
