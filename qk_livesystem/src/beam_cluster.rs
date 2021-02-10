use crate::beam_node::BeamNode;
use qk_term::atom::Atom;
use qk_data::data_stream::t_data_stream::{TDataStream, StreamCaps};
use qk_data::stream_event::StreamEvent;
use qk_data::data_stream::eflame_log::defs::EflameValue;
use qk_term::mfarity::MFArity;
use std::collections::HashMap;

#[derive(Debug)]
pub struct BeamCluster {
  pub nodes: HashMap<Atom, Box<BeamNode>>,
}

impl Default for BeamCluster {
  fn default() -> Self {
    Self::new()
  }
}

impl BeamCluster {
  pub fn new() -> Self {
    // Default start with one unconnected node
    let mut nodes = HashMap::new();
    let node_name = Self::nonode_nohost();
    nodes.insert(node_name, Box::new(BeamNode::new(node_name, false)));

    Self { nodes }
  }

  fn nonode_nohost() -> Atom {
    Atom::new_str("nonode@nohost")
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

    let node_name = Self::nonode_nohost();

    let load_event = |evt: StreamEvent| {
      match evt {
        StreamEvent::ExecuteFunctionEvent(ef) => {
          let mut prev_mfa: Option<MFArity> = None;
          let mut node = self.nodes.get_mut(&node_name).unwrap();

          ef.stack.iter().for_each(|mfa| {
            // assume that this is loaded in single node mode only
            node.code.learned_new_mfa(mfa);

            // Register a call relation, previous mfa calls current mfa in the callstack,
            // except the first, first is ignored as it has no caller.
            if prev_mfa.is_some() {
              node.learned_new_call_relation(&prev_mfa.unwrap(), mfa);
            }
            prev_mfa = Some(*mfa);
          });

          node.learned_new_pid(ef.pid, None)
        }
        e => {
          panic!("Don't know how to import {:?}", e)
        }
      }
    };
    input.get_all().into_iter().for_each(load_event);
  }
}
