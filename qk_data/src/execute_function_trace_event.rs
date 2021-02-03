use qk_term::pid::Pid;

use crate::stream_event::{TStreamEvent, UtcDatetime};
use std::time::Duration;
use qk_term::mfarity::MFArity;

/// Represents event of timed function call with the stack trace.
/// This is usually produced by Eflame-style tracers.
/// Trace-style events might have no start/end time, just the accumulated nanoseconds.
#[derive(Debug)]
pub struct ExecuteFunctionTraceEvent {
  pub pid: Pid,

  /// Call stack of indices into the lookup map of MFArity objects
  pub stack: Vec<MFArity>,

  /// Microseconds spent in this function
  pub time_duration: std::time::Duration,
}

impl ExecuteFunctionTraceEvent {
  pub fn new(pid: Pid, stack: Vec<MFArity>, time_duration: Duration) -> Self {
    ExecuteFunctionTraceEvent { pid, stack, time_duration }
  }
}

impl TStreamEvent for ExecuteFunctionTraceEvent {
  fn start_time(&self) -> Option<UtcDatetime> { None }

  fn end_time(&self) -> Option<UtcDatetime> { None }

  fn duration(&self) -> Duration {
    self.time_duration
  }
}