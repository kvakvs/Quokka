use crate::stream_event::StreamEvent;

bitflags! {
  /// The qualities of data the datastream can provide.
  pub struct StreamCaps: u32 {
    /// Can the data be used for aggregate reports? Produced by Eflame tracing logs.
    /// Not usable for live visualisation.
    const AGGREGATE_EXECUTION_TIME = 0b0001;

    /// Can the data be used for realtime visualisation? Produced by realtime BEAM tracing.
    const REALTIME_BEAM_TRACING = 0b0010;

    /// Historical data able to do rewinding and fast forwarding? Events have start/end time?
    const HAS_START_TIME = 0b0100;

    /// Stream is recorded data, and can be loaded all at once (otherwise will trickle one at a time)
    const HAS_ENTIRE_DATA_READY = 0b1000;
  }
}

/// Declares a source of data for node, process and execution activity data
/// The data is streamed as it is loaded, or as it arrives over the network, or from the live system
/// For example this can be a trace file reader, or a tracing agent installed on the live system.
pub trait TDataStream {
  /// Describe the qualities of the data, this stream can provide
  fn get_capabilities(&self) -> StreamCaps;

  /// Read next event if exists. The engine will interpret it and update the picture accordingly.
  /// This will not be called if get_capabilities() has HasEntireDataReady
  fn get_next(&mut self) -> Option<StreamEvent>;

  /// Read all events.
  /// This will be called if get_capabilities() has HasEntireDataReady
  fn get_all(&mut self) -> Vec<StreamEvent>;
}
