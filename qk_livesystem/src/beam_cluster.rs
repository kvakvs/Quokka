use crate::beam_node::BeamNode;
use qk_term::atom::Atom;
use qk_data::data_stream::t_data_stream::{TDataStream, StreamCaps};
use qk_data::stream_event::StreamEvent;

#[derive(Debug)]
pub struct BeamCluster {
  pub nodes: Vec<BeamNode>,
}

impl BeamCluster {
  pub fn new() -> Self {
    // Default start with one unconnected node
    Self {
      nodes: vec![BeamNode::new(Atom::new_str("nonode@nohost"), false)],
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
    assert_eq!(self.nodes.len(), 1, "Eflame logs can only be loaded into a single node");

    let load_event = |evt: StreamEvent| {
      match evt {
        StreamEvent::ExecuteFunctionEvent(ef) => {
          ef.stack.iter().for_each(|ref mfa| {
            // assume that this is loaded in single node mode only
            self.nodes[0].code.learned_new_mfa(mfa);
            self.nodes[0].learned_new_pid(ef.pid, None)
          })
        }
        e => {
          panic!("Don't know how to import {:?}", e)
        }
      }
    };
    input.get_all().into_iter().for_each(load_event);
  }
}
