#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct AtomIndex(usize);

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct AtomValue(String);

impl AtomValue {
  pub fn new(v: &str) -> Self {
    Self(v.to_string())
  }
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Atom {
  Index(AtomIndex),
  Value(AtomValue),
}

impl Atom {
  pub fn new(v: &str) -> Self {
    Atom::Value(AtomValue::new(v))
  }
}
