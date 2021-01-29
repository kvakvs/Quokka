use qk_term::pid::Pid;
use crate::mfarity::MFArity;

#[derive(Debug, PartialEq)]
pub struct ExecutionTime {
  pub is_sleep: bool,
  pub time: u64,
}

#[derive(Debug, PartialEq)]
pub struct EflameLogLine {
  pid: Pid,
  stack: Vec<MFArity>,
  tail: ExecutionTime,
}
