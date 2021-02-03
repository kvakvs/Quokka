/// QK-TERM cargo package
/// ---------------------
/// Represents parsed Erlang terms for inspection.
///
#[macro_use] extern crate lazy_static;

pub mod term;
pub mod binary;
pub mod pid;
pub mod atom;
pub mod mfarity;
pub mod funarity;

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
