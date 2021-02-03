use qk_term::atom::Atom;

#[derive(Debug)]
pub struct BeamFunction {
  name: Atom,
  arity: u16,
}
