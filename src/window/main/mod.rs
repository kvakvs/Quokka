use std::sync::{Arc, RwLock};

use gtk::prelude::*;

use app::QkApp;
use app_state::QkAppState;

use crate::window::main::app_state::QkViewMode;

pub mod app_state;
pub mod content;
pub mod app;

pub fn start_gui(gtk_app: &gtk::Application) {
  // Set the initial state of our health component. We use an `Arc` so that we can share
  // this value across multiple programmable closures.
  let app_state = Arc::new(RwLock::new(QkAppState::new()));

  // Initialize the UI's initial state.
  let qk_app = QkApp::new(gtk_app, app_state.clone());

  {
    // let st = app_state.clone();
    qk_app.header.btn_cluster.connect_clicked(move |_| {
      QkAppState::set_view_mode(&app_state, QkViewMode::Cluster);
    });
  }

  // Make all the widgets within the UI visible.
  qk_app.window.show_all();
}

pub struct QkMainWindowHeader {
  pub container: gtk::HeaderBar,
  pub btn_cluster: gtk::Button, // Cluster view
}

impl QkMainWindowHeader {
  fn new() -> QkMainWindowHeader {
    let (container, btn_cluster) = QkMainWindowHeader::setup_header_bar();

    // Returns the header and all of it's state
    QkMainWindowHeader {
      container,
      btn_cluster,
    }
  }

  /// Creates the main header bar container widget.
  fn setup_header_bar() -> (gtk::HeaderBar, gtk::Button) {
    let container = gtk::HeaderBar::new();

    // Sets the text to display in the title section of the header bar.
    container.set_title(Some("Quokka Observer"));
    // Enable the window controls within this headerbar.
    container.set_show_close_button(true);

    // Create the hit and heal buttons.
    let btn_cluster = gtk::Button::new();
    btn_cluster.set_label("Cluster View");

    // Add the corresponding style classes to those buttons.
    // btn_cluster.get_style_context().map(|c| c.add_class("destructive-action"));

    // Then add them to the header bar.
    container.pack_start(&btn_cluster);
    (container, btn_cluster)
  }
}

// pub fn create_drawable_window<F>(application: &gtk::Application, sz: Sizei32, draw_fn: F) -> gtk::ApplicationWindow
//   where F: Fn(&gtk::DrawingArea, &cairo::Context) -> Inhibit + 'static,
// {
//   let window = gtk::ApplicationWindow::new(application);
//   let drawing_area = std::boxed::Box::new(gtk::DrawingArea::new)();
//
//   drawing_area.connect_draw(draw_fn);
//   window.set_default_size(sz.x, sz.y);
//   window.add(&drawing_area);
//   window.show_all();
//   window
// }
