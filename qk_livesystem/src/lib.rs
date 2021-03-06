extern crate qk_term;
extern crate qk_data;
extern crate chrono;
// extern crate cairo;
extern crate petgraph;
extern crate rand;

pub mod beam_cluster;
pub mod beam_node;
pub mod beam_process;
pub mod code_server;
pub mod ui;
pub mod force_directed_layout;

use chrono::Utc;

pub type Timestamp = chrono::DateTime<Utc>;

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
