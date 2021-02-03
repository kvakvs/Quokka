use qk_term::atom::Atom;
use std::collections::HashMap;
use qk_term::mfarity::MFArity;
use crate::code_server::beam_function::BeamFunction;

#[derive(Debug)]
pub struct BeamModule {
  name: Atom,

  functions: HashMap<MFArity, BeamFunction>,
}