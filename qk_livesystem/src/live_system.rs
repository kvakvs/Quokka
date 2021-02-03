use crate::beam_node::BeamNode;
use qk_term::atom::Atom;
use crate::code_server::BeamCodeServer;
use qk_data::data_stream::t_data_stream::{TDataStream, StreamCaps};

#[derive(Debug)]
pub struct LiveSystem {
  nodes: Vec<BeamNode>,
  code: Box<BeamCodeServer>,
}

impl LiveSystem {
  pub fn new() -> Self {
    // Default start with one unconnected node
    Self {
      nodes: vec![BeamNode::new(Atom::new_str("nonode@nohost"), false)],
      code: Box::new(BeamCodeServer::new())
    }
  }

  /// Given some sort of TDataStream, read from it and build a picture of the live system.
  pub fn load_data_stream(&mut self, input: Box<dyn TDataStream>) {
    // let tds: &dyn TDataStream = input.as_ref();
    let caps: StreamCaps = input.get_capabilities();
    if caps & StreamCaps::HAS_ENTIRE_DATA_READY != StreamCaps::HAS_ENTIRE_DATA_READY {
      // Entire data not ready
      panic!("Must have all data ready to load_data_stream()");
    }
    println!("Loading from data stream...");
  }
}
