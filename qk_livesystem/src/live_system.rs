use crate::beam_node::BeamNode;
use qk_term::atom::Atom;
use crate::code_server::BeamCodeServer;

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
}
