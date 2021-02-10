use qk_term::atom::Atom;

/// Represents selection of none, one, or some nodes in Code-view
#[derive(Debug)]
pub enum QkCodeSelection {
  None,

  /// One module node is selected
  OneModule(Atom),

  /// Multiple module nodes are selected
  MultipleModules(Vec<Atom>),
}

impl Default for QkCodeSelection {
  fn default() -> Self { QkCodeSelection::None }
}
