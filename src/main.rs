extern crate gtk;
extern crate gdk;
extern crate qk_livesystem;
extern crate cairo;
extern crate gio;

// use gtk::prelude::*;
use gio::prelude::*;

mod window;
mod ui;

fn main() {
  let gtk_application = gtk::Application::new(
    Option::from("se.clau.quokka"),
    gio::ApplicationFlags::FLAGS_NONE,
  ).expect("Gtk Application initialization failed...");

  gtk_application.connect_activate(|app| {
    window::main::start_gui(app);
  });

  gtk_application.run(&[]);
  println!("Finished.");
}

