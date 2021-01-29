#[derive(Debug, PartialOrd, PartialEq)]
pub struct AtomIndex(usize);

#[derive(Debug, PartialOrd, PartialEq)]
pub struct AtomValue(String);

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Atom {
  Index(AtomIndex),
  Value(AtomValue),
}
