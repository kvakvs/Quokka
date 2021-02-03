use qk_term::atom::Atom;
use std::collections::HashMap;
use qk_term::mfarity::MFArity;
use crate::code_server::beam_function::BeamFunction;
use qk_term::funarity::FunArity;

#[derive(Debug)]
pub struct BeamModule {
  name: Atom,

  functions: HashMap<FunArity, BeamFunction>,
}

impl BeamModule {
  pub fn new(name: Atom) -> Self {
    BeamModule {
      name,
      functions: HashMap::new(),
    }
  }

  pub fn learned_new_function(&mut self, mfa: &MFArity) {
    let key = FunArity::new(mfa.fun, mfa.arity);
    self.functions.insert(key, BeamFunction::new(key));
  }
}