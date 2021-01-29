use qk_term::atom::Atom;

/// Full Mod:Fun/Arity representation, to store in a lookup table. Use MFArityIndex instead.
#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct MFArityValue {
  module: Atom,
  fun: Atom,
  arity: u16,
}

impl MFArityValue {
  pub fn new(m: &str, f: &str, arity: u16) -> Self {
    Self {
      module: Atom::new(m),
      fun: Atom::new(f),
      arity
    }
  }
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct MFArityIndex(usize);

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum MFArity {
  Index(MFArityIndex),
  Value(MFArityValue)
}
