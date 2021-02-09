use std::sync::{Arc, RwLock};
use qk_data::data_stream::eflame_log::EflameLogStream;
use qk_livesystem::beam_cluster::BeamCluster;
use qk_livesystem::ui::point::Pointf;
use qk_term::atom::Atom;

use crate::window::main::node_selection::QkNodeSelection;
use crate::window::main::pointer_mode::QkPointerMode;
use crate::window::main::process_selection::QkProcessSelection;

#[allow(dead_code)]
pub enum QkViewMode {
  Cluster,
  Node(Atom),
}

/// Stores application global state: That is current opened project, view mode etc.
pub struct QkApp {
  // TODO: this below belongs to the current project, when projects are introduced
  pub view_mode: QkViewMode,
  pub camera_zoom: f64,
  pub camera_offset: Pointf,
  pub cluster: BeamCluster,

  /// Whether none, one or multiple NODES in the CLUSTER view are selected
  pub node_selection: QkNodeSelection,

  /// Whether none, one or multiple PROCESSES in the NODE view are selected
  pub process_selection: QkProcessSelection,

  /// Whether mouse is busy doing something special like dragging
  pub pointer_mode: QkPointerMode,
}

impl QkApp {
  pub fn new() -> Self {
    Self {
      view_mode: QkViewMode::Cluster,
      camera_zoom: 1.0,
      camera_offset: Pointf::new(0.0, 0.0),
      cluster: Default::default(),
      node_selection: Default::default(),
      process_selection: Default::default(),
      pointer_mode: Default::default(),
    }
  }

  #[allow(dead_code)]
  pub fn load(&mut self) {
    let ef_log = std::boxed::Box::new(EflameLogStream::new("eflame_log.txt").unwrap());
    // ef_log.lines.iter().for_each(|line| { println!("{:?}", line) });

    // Representation of the live system as we know it
    self.cluster.load_data_stream(ef_log);
  }

  // pub fn set_view_mode(this_rw: &RwLock<QkAppState>, view_mode: QkViewMode) {
  //   let mut this = this_rw.write().unwrap();
  //
  //   this.view_mode = view_mode;
  //   // TODO: Redraw the window
  //
  //   drop(this);
  // }

  pub fn read_with<T, TFun>(state_rwlock: &RwLock<Self>, func: TFun) -> T
    where TFun: Fn(&Self) -> T {
    let state = state_rwlock.read().unwrap();
    let result = func(&state);
    drop(state);
    result
  }

  pub fn modify_with<TFun>(state_rwlock: &RwLock<Self>, mut_func: TFun)
    where TFun: Fn(&mut Self) {
    // Type for TFun fn(state: &mut Self) -> ()
    let mut state = state_rwlock.write().unwrap();
    mut_func(&mut state);
    drop(state);
  }
}
