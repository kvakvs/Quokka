use crate::ui::ui_element_state::UiElementState;
use crate::ui::point::Pointf;

pub trait TDrawable {
  fn draw(&self,
          window_offset: Pointf,
          // ui: &mut imgui::Ui,
          draw_list: &imgui::DrawListMut,
          ui_element_state: UiElementState);
}