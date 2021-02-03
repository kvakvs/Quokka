use crate::atom::Atom;
use std::fmt::Debug;
use std::fmt;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub struct FunArity {
  pub fun: Atom,
  pub arity: u16,
}

impl Debug for FunArity {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}/{}", self.fun, self.arity)
  }
}

impl FunArity {
  pub fn new(fun: Atom, arity: u16) -> Self {
    FunArity { fun, arity }
  }
}
