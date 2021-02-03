use qk_term::pid::Pid;

/// Represents a known running BEAM process or which have been running in the past.
#[derive(Debug)]
pub struct BeamProcess {
  pid: Pid,
  linked_to: Vec<Pid>,
  monitoring: Vec<Pid>,
}

impl BeamProcess {
  pub fn new(pid: Pid) -> Self {
    BeamProcess {
      pid,
      linked_to: Vec::new(),
      monitoring: Vec::new(),
    }
  }
}