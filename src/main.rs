extern crate gtk;
extern crate qk_livesystem;
extern crate cairo;
extern crate gio;

use qk_data::data_stream::eflame_log::{parser, EflameLogStream};
use qk_data::data_stream::eflame_log::parser::parse_test;
use qk_livesystem::beam_cluster::BeamCluster;
use gtk::prelude::*;
use gio::prelude::*;
// use std::env;

mod window;
mod my_types;

fn main() {
  let gtk_application = gtk::Application::new(
    "se.clau.quokka",
    gio::APPLICATION_FLAGS_NONE,
  ).expect("Gtk Application initialization failed...");

  gtk_application.connect_activate(|app| {
    window::main::start_gui(app);
  });

  gtk_application.run(&vec![]);
  println!("Finished.");
}

