use crate::atom::Atom;
use std::fmt::Debug;
use std::fmt;

/// Full Mod:Fun/Arity representation, to store in a lookup table. Use MFArityIndex instead.
#[derive(PartialOrd, PartialEq, Clone, Copy)]
pub struct MFArity {
  pub module: Atom,
  pub fun: Atom,
  pub arity: u16,
}

impl Debug for MFArity {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}:{:?}/{}", self.module, self.fun, self.arity)
  }
}

impl MFArity {
  pub fn new(m: &str, f: &str, arity: u16) -> Self {
    Self {
      module: Atom::new(&m.to_string()),
      fun: Atom::new(&f.to_string()),
      arity,
    }
  }

  pub fn new_a(m: Atom, f: Atom, arity: u16) -> Self {
    Self { module: m, fun: f, arity }
  }
}
