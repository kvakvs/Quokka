use crate::binary::Binary;
use crate::pid::Pid;
use crate::atom::Atom;

pub enum Term {
    Int(isize),
    BigInt,
    Float(f64),
    /// Index in the Atom table
    Atom(Atom),
    Tuple(Vec<Term>),
    EmptyTuple,
    List(Vec<Term>),
    EmptyList,
    Binary(Binary),
    Pid(Pid),
}
