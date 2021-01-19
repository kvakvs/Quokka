use crate::stream_event::StreamEvent;

/// Declares a source of data for node, process and execution activity data
/// The data is streamed as it is loaded, or as it arrives over the network, or from the live system
/// For example this can be a trace file reader, or a tracing agent installed on the live system.
pub trait DataStream {
    /// Read next event if exists. The engine will interpret it and update the picture accordingly.
    fn next(&mut self) -> Option<StreamEvent>;
}
