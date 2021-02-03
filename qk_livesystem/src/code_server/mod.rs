use std::collections::HashMap;
use crate::code_server::beam_module::BeamModule;

pub mod beam_module;
pub mod beam_function;

/// Represents loaded code, apps, modules, functions.
#[derive(Debug)]
pub struct BeamCodeServer {
  // otp_apps: HashMap<String, OTPApplication>
  modules: HashMap<String, Box<BeamModule>>,
}

impl BeamCodeServer {
  pub fn new() -> Self {
    BeamCodeServer {
      modules: HashMap::new()
    }
  }
}
