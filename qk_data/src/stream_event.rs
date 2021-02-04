use crate::execute_function_trace_event::ExecuteFunctionTraceEvent;
use crate::send_message_event::SendMessageEvent;

pub type UtcDatetime = chrono::DateTime<chrono::Utc>;

pub trait TStreamEvent {
  fn start_time(&self) -> Option<UtcDatetime>;
  fn end_time(&self) -> Option<UtcDatetime>;
  fn duration(&self) -> std::time::Duration;
}

#[derive(Debug)]
pub enum StreamEvent {
  // Produced by Eflame, only stack traces and execution times
  ExecuteFunctionEvent(ExecuteFunctionTraceEvent),

  // Generic BEAM VM events revealing the changes to the system structure
  SendMessageEvent(SendMessageEvent),
}
