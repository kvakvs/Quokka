extern crate gtk;
extern crate glib;

use std::env::args;

use gio::ApplicationExt;
use gio::prelude::ApplicationExtManual;

mod w_main;

fn main() {
    let application =
        gtk::Application::new(Some("se.clau.quokka"),
                              Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        w_main::build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}