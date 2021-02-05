use crate::ui::ui_element_state::UiElementState;

pub trait TDrawable {
  fn draw(&self, cr: &cairo::Context, ui_element_state: UiElementState);
}