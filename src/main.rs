extern crate gtk;
extern crate qk_livesystem;

use qk_data::data_stream::eflame_log::{parser, EflameLogStream};
use qk_data::data_stream::eflame_log::parser::parse_test;
use qk_livesystem::beam_cluster::BeamCluster;

mod window;

fn main() {
  // parse_test();

  let ef_log = std::boxed::Box::new(EflameLogStream::new("eflame_log.txt").unwrap());
  ef_log.lines.iter().for_each(|line| { println!("{:?}", line) });

  // Representation of the live system as we know it
  let mut livesys = BeamCluster::new();
  livesys.load_data_stream(ef_log);
  println!("{:?}", livesys);

  window::main::start_gui();
}

