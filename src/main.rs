extern crate gtk;
extern crate gdk;
extern crate qk_livesystem;
extern crate cairo;
extern crate gio;
// #[macro_use] extern crate lazy_static;

// use gtk::prelude::*;
use gio::prelude::*;
use window::main::gui::{QkGui};
use crate::app::QkApp;

mod window;
mod ui;
mod app;

fn main() {
  let gtk_application = gtk::Application::new(
    Option::from("se.clau.quokka"),
    gio::ApplicationFlags::FLAGS_NONE,
  ).expect("Gtk Application initialization failed...");

  gtk_application.connect_activate(|gtk_app1| {
    // no return value, no global App variable
    // it all gets distributed into closures attached to widget handlers
    QkApp::setup_qk_app(gtk_app1);
  });

  gtk_application.run(&[]);
  println!("Finished.");
}

