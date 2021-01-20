use qk_term::pid::Pid;

use crate::stream_event::{MFArityIndex, TStreamEvent, UtcDatetime};
use std::time::Duration;

/// Represents event of timed function call with the stack trace.
/// This is usually produced by Eflame-style tracers.
/// Trace-style events might have no start/end time, just the accumulated nanoseconds.
pub struct ExecuteFunctionTraceEvent {
  pid: Pid,

  /// Call stack of indices into the lookup map of MFArity objects
  stack: Vec<MFArityIndex>,

  /// Microseconds spent in this function
  time_duration: std::time::Duration,
}

impl TStreamEvent for ExecuteFunctionTraceEvent {
  fn start_time(&self) -> Option<UtcDatetime> { None }

  fn end_time(&self) -> Option<UtcDatetime> { None }

  fn duration(&self) -> Duration {
    self.time_duration
  }
}