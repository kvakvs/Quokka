pub mod parser;
pub mod defs;

use crate::data_stream::t_data_stream::{TDataStream, DataStreamCapability};
use crate::stream_event::StreamEvent;

struct EflameLogStream {
  filename: String,
}

impl EflameLogStream {
  // fn new(path: &str) -> Self {
  //
  // }
}

impl TDataStream for EflameLogStream {
  fn get_capabilities() -> Vec<DataStreamCapability> {
    vec![DataStreamCapability::HAS_ENTIRE_DATA_READY, ]
  }

  fn get_next(&mut self) -> Option<StreamEvent> {
    unimplemented!() // should not be called, because we are "HasEntireDataReady"
  }

  fn get_all(&mut self) -> Vec<StreamEvent> {
    Vec::new()
  }
}