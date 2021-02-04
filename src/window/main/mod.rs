use std::process;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};

use app_state::QkAppState;
use crate::window::main::app_state::QkViewMode;
use std::borrow::{BorrowMut, Borrow};
use std::ops::{Deref, DerefMut};
use std::f64::consts::PI;
use cairo::{FontSlant, FontWeight, Context};
use gtk::prelude::*;
use gio::prelude::*;

pub mod app_state;

pub fn start_gui(gtk_app: &gtk::Application) {
  // Initialize GTK before proceeding.
  // if gtk::init().is_err() {
  //   eprintln!("failed to initialize GTK Application");
  //   process::exit(1);
  // }

  // Set the initial state of our health component. We use an `Arc` so that we can share
  // this value across multiple programmable closures.
  let mut app_state = Arc::new(RwLock::new(QkAppState::new()));

  // Initialize the UI's initial state.
  let qk_app = App::new(app_state.clone());

  {
    let st = app_state.clone();
    qk_app.header.btn_cluster.connect_clicked(move |_| {
      // let new_health = state.subtract(1);
      // message.set_label("fgsfds");
      // info.set_label(new_health.to_string().as_str());
      QkAppState::set_view_mode(&st, QkViewMode::Cluster);
    });
  }

  // Make all the widgets within the UI visible.
  qk_app.window.show_all();

  create_drawable(gtk_app);

  // Start the GTK main event loop
  // gtk::main();
}

fn create_drawable(application: &gtk::Application) {
  drawable(application, 500, 500, |_, cr| {
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
  });

  drawable(application, 500, 500, |_, cr| {
    cr.scale(500f64, 500f64);

    cr.select_font_face("Sans", FontSlant::Normal, FontWeight::Normal);
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
  });
}

pub struct App {
  pub window: gtk::Window,
  pub header: QkMainWindowHeader,
  pub content: QkMainWindowContent,
  pub app_state: Arc<RwLock<QkAppState>>,
}

pub struct QkMainWindowHeader {
  pub container: gtk::HeaderBar,
  pub btn_cluster: gtk::Button, // Cluster view
}

pub struct QkMainWindowContent {
  pub container: gtk::Box,
  // pub health: gtk::Label,
  // pub message: gtk::Label,
}

impl QkMainWindowContent {
  fn new() -> QkMainWindowContent {
    // Create a vertical box to store all of it's inner children vertically.
    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);

    // The health info will be contained within a horizontal box within the vertical box.
    // let health_info = Box::new(gtk::Orientation::Horizontal, 0);
    // let health_label = gtk::Label::new(Some("Current Health:"));
    // let health = gtk::Label::new(Some(health.get_health().to_string().as_str()));

    // Set the horizontal alignments of each of our objects.
    // health_info.set_halign(Align::Center);
    // health_label.set_halign(Align::Start);
    // health.set_halign(Align::Start);


    // Add the health info box's children
    // health_info.pack_start(&health_label, false, false, 5);
    // health_info.pack_start(&health, true, true, 5);

    // Create a message label that will later be modified by the application, upon
    // performing a hit or heal action.
    let message = gtk::Label::new(Some("Hello"));

    // Add everything to our vertical box
    // container.pack_start(&health_info, true, false, 0);
    // container.pack_start(&Separator::new(gtk::Orientation::Horizontal), false, false, 0);
    // container.pack_start(&message, true, false, 0);

    QkMainWindowContent { container }
  }
}

impl App {
  fn new(app_state: Arc<RwLock<QkAppState>>) -> App {
    // Create a new top level window.
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    // Create a the headerbar and it's associated content.
    let header = QkMainWindowHeader::new();
    // Contains the content within the window.
    let content = QkMainWindowContent::new();

    // Set the headerbar as the title bar widget.
    window.set_titlebar(Some(&header.container));
    // Set the title of the window.
    window.set_title("App Name");
    // Set the window manager class.
    window.set_wmclass("app-name", "App name");
    // The icon the app will display.
    gtk::Window::set_default_icon_name("iconname");
    // Add the content box into the window.
    window.add(&content.container);

    // Programs what to do when the exit button is used.
    window.connect_delete_event(move |_, _| {
      gtk::main_quit();
      gtk::Inhibit(false)
    });

    // Return our main application state
    App {
      window,
      header,
      content,
      app_state,
    }
  }
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
    let btn_cluster = gtk::Button::new_with_label("Cluster View");

    // Add the corresponding style classes to those buttons.
    // btn_cluster.get_style_context().map(|c| c.add_class("destructive-action"));

    // Then add them to the header bar.
    container.pack_start(&btn_cluster);
    (container, btn_cluster)
  }
}

pub fn drawable<F>(application: &gtk::Application, width: i32, height: i32, draw_fn: F)
  where
      F: Fn(&gtk::DrawingArea, &Context) -> Inhibit + 'static,
{
  let window = gtk::ApplicationWindow::new(application);
  let drawing_area = std::boxed::Box::new(gtk::DrawingArea::new)();

  drawing_area.connect_draw(draw_fn);

  window.set_default_size(width, height);

  window.add(&drawing_area);
  window.show_all();
}
