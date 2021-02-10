use std::collections::HashMap;

use qk_term::atom::Atom;
use qk_term::pid::Pid;
use imgui::*;

use crate::beam_process::BeamProcess;
use crate::code_server::BeamCodeServer;
use crate::Timestamp;
use crate::ui::draw::TDrawable;
use crate::ui::layout::{Layout};
use crate::ui::point::Pointf;
use crate::ui::size::Sizef;
use crate::ui::ui_element_state::UiElementState;
use qk_term::mfarity::MFArity;
use petgraph::graph::{Node, NodeIndex};

#[derive(Debug)]
pub struct BeamNode {
  // Distribution and node name
  pub name: Atom,
  hidden: bool,
  connected_to: Vec<Atom>,
  connected_to_all: bool,

  /// UI section: positioning, classification, tags, colors, grouping
  layout: Layout,

  /// Static (more or less static) resources, such as code structure
  pub code: BeamCodeServer,

  /// Processes
  processes: HashMap<Pid, BeamProcess>,

  /// Graph for function-call-function relations
  rel_call: petgraph::Graph::<MFArity, (), petgraph::Directed>,
}

// impl TLayout for BeamNode {
//   fn layout_pos(&self) -> &Pointf { &self.layout.pos }
//   fn layout_pos_mut(&mut self) -> &mut Pointf { &mut self.layout.pos }
//   fn layout_size(&self) -> &Option<Sizef> { &self.layout.size }
//   fn layout_size_mut(&mut self) -> &mut Option<Sizef> { &mut self.layout.size }
// }

impl BeamNode {
  pub fn new(name: Atom, hidden: bool) -> Self {
    BeamNode {
      name,
      hidden,
      code: BeamCodeServer::new(),
      connected_to: Vec::new(),
      connected_to_all: false,
      processes: HashMap::new(),
      layout: Layout::new(Pointf::new_random()),
      rel_call: petgraph::Graph::new(),
    }
  }

  /// Used when loading or reading data stream from the live node or logs.
  /// This lets BeamNode know that a process with this pid has been found. No extra data is stored
  /// with that process at this point.
  pub fn learned_new_pid(&mut self, pid: Pid, when: Option<Timestamp>) {
    assert_eq!(when, None);
    self.processes.insert(pid, BeamProcess::new(pid, when));
  }

  fn add_rel_call_unique_node(&mut self, new_node: &MFArity) -> NodeIndex {
    match self.rel_call.raw_nodes().iter().enumerate()
        .find(|(i, n)| {
          n.weight == *new_node
        })
    {
      None => { self.rel_call.add_node(*new_node) }
      Some((i, n)) => { NodeIndex::new(i) }
    }
  }

  /// Used when loading or reading data stream from the live node or logs.
  pub fn learned_new_call_relation(&mut self, caller: &MFArity, call_target: &MFArity) {
    let a = self.add_rel_call_unique_node(caller);
    let b = self.add_rel_call_unique_node(call_target);
    self.rel_call.add_edge(a, b, ());
  }

  pub fn is_mouse_hit(&self, mouse: &Pointf) -> bool {
    self.layout.draw_box.contains_point(mouse)
  }
}

impl TDrawable for BeamNode {
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