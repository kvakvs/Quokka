use std::collections::HashMap;
use crate::code_server::beam_module::BeamModule;
use qk_term::mfarity::MFArity;
use qk_term::atom::Atom;

pub mod beam_module;
pub mod beam_function;

/// Represents loaded code, apps, modules, functions.
#[derive(Debug)]
pub struct BeamCodeServer {
  // otp_apps: HashMap<String, OTPApplication>
  modules: HashMap<Atom, Box<BeamModule>>,
}

impl BeamCodeServer {
  pub fn new() -> Self {
    BeamCodeServer {
      modules: HashMap::new()
    }
  }

  pub fn get_or_create_module(&mut self, module: Atom) -> &mut Box<BeamModule> {
    self.modules.entry(module).or_insert(Box::new(BeamModule::new(module)))
    //   None => {
    //     let new_mod = ;
    //     self.modules[module] = new_mod.clone();
    //     new_mod
    //   }
    //   Some(mod_box) => mod_box.clone(),
    // }
  }

  /// Inform the Code Server, that some MFA exists in the code (been recently used or learned from
  /// some trace logs). This will be displayed in the code map.
  pub fn learned_new_mfa(&mut self, mfa: &MFArity) {
    let m = self.get_or_create_module(mfa.module);
    m.learned_new_function(&mfa);
  }
}
