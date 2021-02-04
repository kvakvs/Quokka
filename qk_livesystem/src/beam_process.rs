use qk_term::pid::Pid;
use crate::Timestamp;

/// Represents a known running BEAM process or which have been running in the past.
#[derive(Debug)]
pub struct BeamProcess {
  created_at: Option<Timestamp>,
  pid: Pid,
  linked_to: Vec<Pid>,
  monitoring: Vec<Pid>,
}

impl BeamProcess {
  pub fn new(pid: Pid, created_at: Option<Timestamp>) -> Self {
    BeamProcess {
      pid,
      created_at,
      linked_to: Vec::new(),
      monitoring: Vec::new(),
    }
  }
}