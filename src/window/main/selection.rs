use qk_term::atom::Atom;

/// Represents selected object or objects in Quokka view
pub enum QkSelection {
  None,
  Node(Atom),
}

impl Default for QkSelection {
  fn default() -> Self { QkSelection::None }
}
