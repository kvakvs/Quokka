/// QK-DATA cargo package
/// ---------------------
/// Represents data events flowing from the nodes, or out of trace files.
/// Data events help Quokka build the visible picture of the system under the investigation.
///
extern crate qk_term;
extern crate chrono;
#[macro_use] extern crate bitflags;
extern crate nom;

pub mod stream_event;
pub mod data_stream;
pub mod send_message_event;
pub mod execute_function_trace_event;

#[cfg(test)]
mod tests {
  use crate::stream_event::StreamEvent;

  #[test]
  fn try_create_event() {
    let se1 = StreamEvent::SendMessageEvent(p1, p2, "");
  }
}
