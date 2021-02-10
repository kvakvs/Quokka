use std::collections::HashMap;
use crate::code_server::beam_module::BeamModule;
use qk_term::mfarity::MFArity;
use qk_term::atom::Atom;

pub mod beam_module;
pub mod beam_function;

/// Represents loaded code, apps, modules, functions.
#[derive(Debug)]
pub struct BeamCodeServer {
  // TODO: otp_apps: HashMap<String, OTPApplication>
  pub modules: HashMap<Atom, Box<BeamModule>>,
}

impl Default for BeamCodeServer {
  fn default() -> Self {
    Self::new()
  }
}

impl BeamCodeServer {
  pub fn new() -> Self {
    BeamCodeServer {
      modules: HashMap::new()
    }
  }

  pub fn get_or_create_module(&mut self, module: Atom) -> &mut Box<BeamModule> {
    self.modules.entry(module).or_insert_with(|| Box::new(BeamModule::new(module)))
  }

  /// Inform the Code Server, that some MFA exists in the code (been recently used or learned from
  /// some trace logs). This will be displayed in the code map.
  pub fn learned_new_mfa(&mut self, mfa: &MFArity) {
    let m = self.get_or_create_module(mfa.module);
    m.learned_new_function(&mfa);
  }
}
