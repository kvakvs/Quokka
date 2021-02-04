use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::RwLock;
use qk_term::atom::Atom;
use std::ops::DerefMut;

pub enum QkViewMode {
  Cluster,
  Node(Atom),
}

/// Stores application global state: That is current opened project, view mode etc.
pub struct QkAppState {
  lock: RwLock<()>,

  // TODO: this below belongs to the current project, when projects are introduced
  view_mode: QkViewMode,
}

impl QkAppState {
  pub fn new() -> Self {
    Self {
      lock: RwLock::new(()),
      view_mode: QkViewMode::Cluster,
    }
  }

  pub fn set_view_mode(&mut self, view_mode: QkViewMode) {
    let lock = self.lock.write().unwrap();

    self.view_mode = view_mode;
    // TODO: Redraw the window

    drop(lock);
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
