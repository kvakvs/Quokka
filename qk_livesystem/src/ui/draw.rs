use crate::ui::ui_element_state::UiElementState;

pub trait TDrawable {
  fn draw(&self, ui: &imgui::Ui, ui_element_state: UiElementState);
}