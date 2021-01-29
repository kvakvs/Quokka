use qk_term::atom::Atom;
use qk_term::pid::Pid;
use qk_term::term::Term;

use crate::execute_function_trace_event::ExecuteFunctionTraceEvent;
use crate::send_message_event::SendMessageEvent;

pub type UtcDatetime = chrono::DateTime<chrono::Utc>;

pub trait TStreamEvent {
  fn start_time(&self) -> Option<UtcDatetime>;
  fn end_time(&self) -> Option<UtcDatetime>;
  fn duration(&self) -> std::time::Duration;
}

pub enum StreamEvent {
  ExecuteFunctionEvent(ExecuteFunctionTraceEvent),
  SendMessageEvent(SendMessageEvent),
}
