use qk_term::atom::Atom;

/// Represents selection of none, one, or some nodes in Cluster-view
pub enum QkNodeSelection {
  None,

  /// One node is selected
  One(Atom),

  /// Multiple nodes are selected
  Multiple(Vec<Atom>),
}

impl Default for QkNodeSelection {
  fn default() -> Self { QkNodeSelection::None }
}
