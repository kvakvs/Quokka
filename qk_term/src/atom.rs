use std::collections::{HashMap};
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::ops::Index;

lazy_static! {
    static ref ATOM_INDEX: AtomicUsize = AtomicUsize::new(0);
    static ref ATOM_STR_TO_INDEX: RwLock<HashMap<String, usize>> = RwLock::new(HashMap::new());
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Atom(usize);

impl Atom {
  pub fn new(v: String) -> Self {
    // Try read-lock the global and find the key in it
    let r_dict = ATOM_STR_TO_INDEX.read().unwrap();
    let find_result = r_dict.get(&v);

    match find_result {
      // If the key is not found, we create new atom index and store the atom name there
      None => {
        let i = ATOM_INDEX.fetch_add(1, Ordering::SeqCst);

        drop(r_dict);

        let mut dict = ATOM_STR_TO_INDEX.write().unwrap();
        dict.insert(v, i);
        drop(dict);

        Atom(i)
      }
      // If the key is already in the atom table, we build an atom from the known index
      Some(&index) => {
        drop(r_dict);
        Atom(index)
      }
    }
  }
}
