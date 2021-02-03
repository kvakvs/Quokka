use qk_term::atom::Atom;
use std::collections::HashMap;
use qk_term::pid::Pid;
use crate::beam_process::BeamProcess;

#[derive(Debug)]
pub struct BeamNode {
  // Distribution and node name
  name: Atom,
  hidden: bool,
  connected_to: Vec<Atom>,
  connected_to_all: bool,

  // Processes
  processes: HashMap<Pid, BeamProcess>,
}

impl BeamNode {
  pub fn new(name: Atom, hidden: bool) -> Self {
    BeamNode {
      name,
      hidden,
      connected_to: Vec::new(),
      connected_to_all: false,
      processes: HashMap::new(),
    }
  }
}
