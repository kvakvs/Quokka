use qk_term::pid::Pid;

/// Represents selection of none, one, or some nodes in Cluster-view
pub enum QkProcessSelection {
  None,

  /// One node is selected
  One(Pid),

  /// Multiple nodes are selected
  Multiple(Vec<Pid>),
}

impl Default for QkProcessSelection {
  fn default() -> Self { QkProcessSelection::None }
}
