use qk_term::atom::Atom;
use std::collections::HashMap;
use qk_term::pid::Pid;
use crate::beam_process::BeamProcess;
use crate::code_server::BeamCodeServer;
use crate::Timestamp;
use crate::ui::{TLayout, Layout, Sizef, Pointf};

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
      layout: Layout::new(Pointf::new(10.0, 20.0)),
    }
  }

  pub fn learned_new_pid(&mut self, pid: Pid, when: Option<Timestamp>) {
    assert_eq!(when, None);
    self.processes.insert(pid, BeamProcess::new(pid, when));
  }
}
