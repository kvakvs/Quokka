use imgui::*;
use qk_term::atom::Atom;
use std::collections::HashMap;
use qk_term::mfarity::MFArity;
use crate::code_server::beam_function::BeamFunction;
use qk_term::funarity::FunArity;
use crate::ui::layout::Layout;
use crate::ui::point::Pointf;
use crate::ui::draw::TDrawable;
use crate::ui::ui_element_state::UiElementState;
use crate::ui::size::Sizef;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct BeamModule {
  name: Atom,

  /// UI section: positioning, classification, tags, colors, grouping
  pub layout: Layout,

  /// Whether the renderer will show the functions inside, or just the module itself
  editor_unfold: bool,

  functions: HashMap<FunArity, BeamFunction>,
}

impl BeamModule {
  pub fn new(name: Atom) -> Self {
    BeamModule {
      name,
      layout: Layout::new(Pointf::new_random()),
      editor_unfold: false,
      functions: HashMap::new(),
    }
  }

  pub fn learned_new_function(&mut self, mfa: &MFArity) {
    let key = FunArity::new(mfa.fun, mfa.arity);
    self.functions.insert(key, BeamFunction::new(key));
  }
}

impl TDrawable for BeamModule {
  fn draw(&self,
          window_offset: Pointf,
          // ui: &mut imgui::Ui,
          draw_list: &imgui::DrawListMut,
          ui_element_state: UiElementState) {
    const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    const SELECTED_COLOR: [f32; 4] = [0.4, 0.7, 1.0, 1.0];

    let draw_color = match ui_element_state {
      UiElementState::Normal => { WHITE }
      UiElementState::Selected => { SELECTED_COLOR }
    };

    draw_list.add_rect((self.layout.draw_box.start + window_offset).into(),
                       (self.layout.draw_box.end + window_offset).into(),
                       draw_color)
        .thickness(2.0)
        .build();

    // // Draw the box
    // {
    //   styling::line_style::apply(cr, ui_element_state);
    //   cr.rectangle(origin.x, origin.y, sz.x, sz.y);
    //   cr.stroke();
    // }
    //
    // // Draw a text node name label under the box
    // {
    let label = self.name.get_str().unwrap_or_else(|| "?".to_string());
    let rect = Sizef::new(32.0, 16.0);
    let text_start = Pointf::new(
      self.layout.draw_box.start.x - rect.x * 0.5,
      self.layout.draw_box.end.y + rect.y,
    ) + window_offset;
    draw_list.add_text(text_start.into(), draw_color, ImString::from(label));
    //
    //   // Draw background under the label if selected
    //   styling::font_style::FONT_SELECTED_BACKGROUND.fill_rectangle(
    //     cr, text_start_x, text_start_y - rect.height, rect.width, rect.height);
    //
    //   // Draw text
    //   styling::font_style::apply(cr, ui_element_state);
    //   cr.move_to(text_start_x, text_start_y);
    //   cr.show_text(&label);
    // }
  }
}
