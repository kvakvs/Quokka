use qk_term::atom::Atom;
use std::collections::HashMap;
use qk_term::pid::Pid;
use crate::beam_process::BeamProcess;
use crate::code_server::BeamCodeServer;
use crate::Timestamp;

#[derive(Debug)]
pub struct BeamNode {
  // Distribution and node name
  name: Atom,
  hidden: bool,
  connected_to: Vec<Atom>,
  connected_to_all: bool,

  // Static (more or less static) resources, such as code structure
  pub(crate) code: Box<BeamCodeServer>,

  // Processes
  processes: HashMap<Pid, BeamProcess>,
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
    }
  }

  pub fn learned_new_pid(&mut self, pid: Pid, when: Option<Timestamp>) {
    assert_eq!(when, None);
    self.processes.insert(pid, BeamProcess::new(pid, when));
  }
}
