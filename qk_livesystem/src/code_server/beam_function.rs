use qk_term::atom::Atom;
use std::fmt::Debug;
use qk_term::funarity::FunArity;
use std::fmt;

pub struct BeamFunction {
  funarity: FunArity,
}

impl Debug for BeamFunction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "(fun {:?})", self.funarity)
  }
}

impl BeamFunction {
  pub fn new(fa: FunArity) -> Self {
    BeamFunction { funarity: fa }
  }
}
