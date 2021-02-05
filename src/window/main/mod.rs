use std::borrow::{Borrow, BorrowMut};
use std::f64::consts::PI;
use std::ops::{Deref, DerefMut};
use std::process;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};

use gio::prelude::*;
use gtk::prelude::*;

use app::QkApp;
use app_state::QkAppState;
use content::QkMainWindowContent;

use crate::my_types::{Size, Sizei32};
use crate::window::main::app_state::QkViewMode;

pub mod app_state;
pub mod content;
pub mod app;

pub fn start_gui(gtk_app: &gtk::Application) {
  // Set the initial state of our health component. We use an `Arc` so that we can share
  // this value across multiple programmable closures.
  let mut app_state = Arc::new(RwLock::new(QkAppState::new()));

  // Initialize the UI's initial state.
  let qk_app = QkApp::new(gtk_app, app_state.clone());

  {
    let st = app_state.clone();
    qk_app.header.btn_cluster.connect_clicked(move |_| {
      QkAppState::set_view_mode(&st, QkViewMode::Cluster);
    });
  }

  // Make all the widgets within the UI visible.
  qk_app.window.show_all();
}

fn create_drawable1(application: &gtk::Application) -> gtk::ApplicationWindow {
  create_drawable_window(
    application,
    Sizei32::new(500, 500),
    |_da: &gtk::DrawingArea, cr: &cairo::Context| {
      cr.set_dash(&[3., 2., 1.], 1.);
      assert_eq!(cr.get_dash(), (vec![3., 2., 1.], 1.));

      cr.scale(500f64, 500f64);

      cr.set_source_rgb(250.0 / 255.0, 224.0 / 255.0, 55.0 / 255.0);
      cr.paint();

      cr.set_line_width(0.05);

      // border
      cr.set_source_rgb(0.3, 0.3, 0.3);
      cr.rectangle(0.0, 0.0, 1.0, 1.0);
      cr.stroke();

      cr.set_line_width(0.03);

      // draw circle
      cr.arc(0.5, 0.5, 0.4, 0.0, PI * 2.);
      cr.stroke();

      // mouth
      let mouth_top = 0.68;
      let mouth_width = 0.38;

      let mouth_dx = 0.10;
      let mouth_dy = 0.10;

      cr.move_to(0.50 - mouth_width / 2.0, mouth_top);
      cr.curve_to(
        0.50 - mouth_dx,
        mouth_top + mouth_dy,
        0.50 + mouth_dx,
        mouth_top + mouth_dy,
        0.50 + mouth_width / 2.0,
        mouth_top,
      );

      println!("Extents: {:?}", cr.fill_extents());

      cr.stroke();

      let eye_y = 0.38;
      let eye_dx = 0.15;
      cr.arc(0.5 - eye_dx, eye_y, 0.05, 0.0, PI * 2.);
      cr.fill();

      cr.arc(0.5 + eye_dx, eye_y, 0.05, 0.0, PI * 2.);
      cr.fill();

      Inhibit(false)
    })
}

fn create_drawable2(application: &gtk::Application) -> gtk::ApplicationWindow {
  create_drawable_window(
    application,
    Sizei32::new(500, 500),
    |_da: &gtk::DrawingArea, cr: &cairo::Context| {
      cr.scale(500f64, 500f64);

      cr.select_font_face("Sans",
                          cairo::FontSlant::Normal,
                          cairo::FontWeight::Normal);
      cr.set_font_size(0.35);

      cr.move_to(0.04, 0.53);
      cr.show_text("Hello");

      cr.move_to(0.27, 0.65);
      cr.text_path("void");
      cr.set_source_rgb(0.5, 0.5, 1.0);
      cr.fill_preserve();
      cr.set_source_rgb(0.0, 0.0, 0.0);
      cr.set_line_width(0.01);
      cr.stroke();

      cr.set_source_rgba(1.0, 0.2, 0.2, 0.6);
      cr.arc(0.04, 0.53, 0.02, 0.0, PI * 2.);
      cr.arc(0.27, 0.65, 0.02, 0.0, PI * 2.);
      cr.fill();

      Inhibit(false)
    })
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

pub fn create_drawable_window<F>(application: &gtk::Application, sz: Size<i32>, draw_fn: F) -> gtk::ApplicationWindow
  where F: Fn(&gtk::DrawingArea, &cairo::Context) -> Inhibit + 'static,
{
  let window = gtk::ApplicationWindow::new(application);
  let drawing_area = std::boxed::Box::new(gtk::DrawingArea::new)();

  drawing_area.connect_draw(draw_fn);
  window.set_default_size(sz.x, sz.y);
  window.add(&drawing_area);
  window.show_all();
  window
}
