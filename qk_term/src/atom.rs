use std::collections::{HashMap};
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::ops::Index;
use std::fmt::Debug;
use std::fmt;
use std::io::Read;

struct AtomIndex {
  pub next_index: usize,
  pub str_to_index: HashMap<String, usize>,
  pub index_to_str: Vec<String>,
}

lazy_static! {
    static ref ATOM_INDEX: RwLock<AtomIndex> = RwLock::new(AtomIndex {
      next_index: 0,
      str_to_index: HashMap::new(),
      index_to_str: Vec::new()
    });
}

impl AtomIndex {
  pub fn find_index(k: &String) -> Option<usize> {
    let r_lock = ATOM_INDEX.read().unwrap();
    match r_lock.str_to_index.get_key_value(k) {
      None => {
        drop(r_lock);
        None
      },
      Some((_, index)) => {
        let result = Some(*index);
        drop(r_lock);
        result
      }
    }
  }

  pub fn get_str(i: usize) -> Option<String> {
    let readable_index = ATOM_INDEX.read().unwrap();
    if readable_index.index_to_str.len() > i {
      let result = Some(readable_index.index_to_str[i].clone());
      drop(readable_index);
      result
    } else {
      drop(readable_index);
      None
    }
  }

  pub fn register_atom(k: String) -> usize {
    let mut writeable_index = ATOM_INDEX.write().unwrap();
    let next_index = writeable_index.next_index;
    writeable_index.next_index += 1;

    writeable_index.str_to_index.insert(k.clone(), next_index);

    assert_eq!(next_index, writeable_index.index_to_str.len());
    writeable_index.index_to_str.push(k);

    // Release
    drop(writeable_index);
    next_index
  }
}

#[derive(PartialOrd, PartialEq, Clone)]
pub struct Atom(usize);

impl Debug for Atom {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.get_str() {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Atom(badindex, {})", self.0),
    }
  }
}

impl Atom {
  pub fn new(v: &String) -> Self {
    match AtomIndex::find_index(&v) {
      None => {
        // If the key is not found, we create new atom index and store the atom name there
        let i = AtomIndex::register_atom(v.clone());
        Atom(i)
      }
      Some(found_index) => {
        // If the key is already in the atom table, we build an atom from the known index
        Atom(found_index)
      }
    }
  }

  pub fn get_str(&self) -> Option<String> {
    AtomIndex::get_str(self.0)
  }
}
