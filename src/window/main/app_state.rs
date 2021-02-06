use std::sync::RwLock;
use qk_term::atom::Atom;
use qk_livesystem::beam_cluster::BeamCluster;
use qk_data::data_stream::eflame_log::EflameLogStream;
use qk_livesystem::ui::point::Pointf;
use crate::window::main::selection::QkSelection;

#[allow(dead_code)]
pub enum QkViewMode {
  Cluster,
  Node(Atom),
}

/// Stores application global state: That is current opened project, view mode etc.
pub struct QkAppState {
  // TODO: this below belongs to the current project, when projects are introduced
  pub view_mode: QkViewMode,
  pub camera_zoom: f64,
  pub camera_offset: Pointf,
  pub cluster: BeamCluster,
  pub selected_object: QkSelection,
}

impl QkAppState {
  pub fn new() -> Self {
    Self {
      view_mode: QkViewMode::Cluster,
      camera_zoom: 1.0,
      camera_offset: Pointf::new(0.0, 0.0),
      cluster: BeamCluster::new(),
      selected_object: Default::default(),
    }
  }

  #[allow(dead_code)]
  pub fn load(&mut self) {
    let ef_log = std::boxed::Box::new(EflameLogStream::new("eflame_log.txt").unwrap());
    // ef_log.lines.iter().for_each(|line| { println!("{:?}", line) });

    // Representation of the live system as we know it
    self.cluster.load_data_stream(ef_log);
  }

  pub fn set_view_mode(this_rw: &RwLock<QkAppState>, view_mode: QkViewMode) {
    let mut this = this_rw.write().unwrap();

    this.view_mode = view_mode;
    // TODO: Redraw the window

    drop(this);
  }

  // pub fn get_health(&self) -> usize { self.0.load(Ordering::SeqCst) }

  // pub fn subtract(&self, value: usize) -> usize {
  //   let current = self.0.load(Ordering::SeqCst);
  //   let new = if current < value { 0 } else { current - value };
  //   self.0.store(new, Ordering::SeqCst);
  //   new
  // }


  // fn heal(&self, value: usize) -> usize {
  //   let original = self.0.fetch_add(value, Ordering::SeqCst);
  //   original + value
  // }
}
