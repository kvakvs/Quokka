pub mod parser;
pub mod defs;
pub mod errors;

use crate::data_stream::t_data_stream::{TDataStream, StreamCaps};
use crate::stream_event::StreamEvent;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use crate::data_stream::eflame_log::defs::EflameLogLine;
use crate::data_stream::eflame_log::parser::parse_eflame_log_line;
use crate::data_stream::eflame_log::errors::EflameError;
use crate::stream_event::StreamEvent::ExecuteFunctionEvent;
use crate::execute_function_trace_event::ExecuteFunctionTraceEvent;

#[derive(Debug)]
pub struct EflameLogStream {
  pub filename: String,
  pub lines: Vec<EflameLogLine>,
}

impl EflameLogStream {
  pub fn new(path: &str) -> Result<Self, EflameError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut parsed_lines = Vec::<EflameLogLine>::new();

    for line in reader.lines() {
      if line.is_ok() {
        match parse_eflame_log_line(line?.as_str()) {
          Ok((_tail, line)) => {
            parsed_lines.push(line);
          }
          Err(e) => {
            return Err(EflameError::from(e));
          }
        }
      }
    } // for loop

    Ok(Self {
      filename: path.to_string(),
      lines: parsed_lines,
    })
  }
}

impl TDataStream for EflameLogStream {
  fn get_capabilities(&self) -> StreamCaps {
    StreamCaps::HAS_ENTIRE_DATA_READY
  }

  fn get_next(&mut self) -> Option<StreamEvent> {
    unimplemented!() // should not be called, because we are "HasEntireDataReady"
  }

  /// Convert the loaded events in EflameLogStream into a vec of StreamEvents
  fn get_all(&self) -> Vec<StreamEvent> {
    self.lines
        .iter()
        .map(|ref line| -> StreamEvent {
          let ef_event = ExecuteFunctionTraceEvent {
            pid: line.pid,
            stack: line.stack.clone(),
            time_duration: std::time::Duration::from_micros(line.tail.time),
          };
          StreamEvent::ExecuteFunctionEvent(ef_event)
        })
        .collect()
  }
}
