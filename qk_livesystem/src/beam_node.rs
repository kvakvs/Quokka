use std::collections::HashMap;

use qk_term::atom::Atom;
use qk_term::pid::Pid;

use crate::beam_process::BeamProcess;
use crate::code_server::BeamCodeServer;
use crate::Timestamp;
use crate::ui::layout::{Layout, TLayout};
use crate::ui::point::Pointf;
use crate::ui::size::Sizef;
use crate::ui::draw::TDrawable;
use cairo::Context;
use crate::ui::ui_element_state::UiElementState;

#[derive(Debug)]
pub struct BeamNode {
  // Distribution and node name
  name: Atom,
  hidden: bool,
  connected_to: Vec<Atom>,
  connected_to_all: bool,

  // UI section: positioning, classification, tags, colors, grouping
  layout: Layout,

  // Static (more or less static) resources, such as code structure
  pub(crate) code: Box<BeamCodeServer>,

  // Processes
  processes: HashMap<Pid, BeamProcess>,
}

impl TLayout for BeamNode {
  fn layout_pos(&self) -> &Pointf { &self.layout.pos }
  fn layout_pos_mut(&mut self) -> &mut Pointf { &mut self.layout.pos }
  fn layout_size(&self) -> &Option<Sizef> { &self.layout.size }
  fn layout_size_mut(&mut self) -> &mut Option<Sizef> { &mut self.layout.size }
}

impl BeamNode {
  pub fn new(name: Atom, hidden: bool) -> Self {
    BeamNode {
      name,
      hidden,
      code: Box::new(BeamCodeServer::new()),
      connected_to: Vec::new(),
      connected_to_all: false,
      processes: HashMap::new(),
      layout: Layout::new(Pointf::new(40.0, 30.0)),
    }
  }

  pub fn learned_new_pid(&mut self, pid: Pid, when: Option<Timestamp>) {
    assert_eq!(when, None);
    self.processes.insert(pid, BeamProcess::new(pid, when));
  }
}

impl TDrawable for BeamNode {
  fn draw(&self, cr: &Context, elst: UiElementState) {
    let sz = self.layout.size.unwrap_or(Sizef::new(20.0, 20.0));
    let origin = Pointf::new(self.layout.pos.x - 0.5 * sz.x,
                             self.layout.pos.y - 0.5 * sz.y);

    cr.set_source_rgb(0.3, 0.3, 0.3);
    cr.rectangle(origin.x, origin.y, sz.x, sz.y);
    cr.stroke();

    const FONT_HEIGHT: f64 = 12.0;
    cr.set_font_size(FONT_HEIGHT);

    // Draw a text node name label under the box
    let label = self.name.get_str().unwrap_or("?".to_string());
    let label_ext = cr.text_extents(&label);

    if elst == UiElementState::Selected {
      cr.fi();
    }

    cr.move_to(origin.x - label_ext.width * 0.5 - label_ext.x_bearing,
               origin.y + sz.y + label_ext.height);
    cr.show_text(&label);
  }
}