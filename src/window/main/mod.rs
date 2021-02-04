use std::process;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};

use gtk::*;

use app_state::QkAppState;
use crate::window::main::app_state::QkViewMode;
use std::borrow::{BorrowMut, Borrow};
use std::ops::{Deref, DerefMut};

pub mod app_state;

pub fn start_gui() {
  // Initialize GTK before proceeding.
  if gtk::init().is_err() {
    eprintln!("failed to initialize GTK Application");
    process::exit(1);
  }

  // Set the initial state of our health component. We use an `Arc` so that we can share
  // this value across multiple programmable closures.
  let mut app_state = Arc::new(RwLock::new(QkAppState::new()));

  // Initialize the UI's initial state.
  let app = App::new(app_state.clone());

  {
    let st = app_state.clone();
    app.header.btn_cluster.connect_clicked(move |_| {
      // let new_health = state.subtract(1);
      // message.set_label("fgsfds");
      // info.set_label(new_health.to_string().as_str());
      QkAppState::set_view_mode(&st, QkViewMode::Cluster);
    });
  }

  // Make all the widgets within the UI visible.
  app.window.show_all();

  // Start the GTK main event loop
  gtk::main();
}

pub struct App {
  pub window: Window,
  pub header: Header,
  pub content: Content,
  pub app_state: Arc<RwLock<QkAppState>>,
}

pub struct Header {
  pub container: HeaderBar,
  pub btn_cluster: Button, // Cluster view
}

pub struct Content {
  pub container: Box,
  // pub health: Label,
  // pub message: Label,
}

impl Content {
  fn new() -> Content {
    // Create a vertical box to store all of it's inner children vertically.
    let container = Box::new(Orientation::Vertical, 0);

    // The health info will be contained within a horizontal box within the vertical box.
    // let health_info = Box::new(Orientation::Horizontal, 0);
    // let health_label = Label::new(Some("Current Health:"));
    // let health = Label::new(Some(health.get_health().to_string().as_str()));

    // Set the horizontal alignments of each of our objects.
    // health_info.set_halign(Align::Center);
    // health_label.set_halign(Align::Start);
    // health.set_halign(Align::Start);


    // Add the health info box's children
    // health_info.pack_start(&health_label, false, false, 5);
    // health_info.pack_start(&health, true, true, 5);

    // Create a message label that will later be modified by the application, upon
    // performing a hit or heal action.
    let message = Label::new(Some("Hello"));

    // Add everything to our vertical box
    // container.pack_start(&health_info, true, false, 0);
    // container.pack_start(&Separator::new(Orientation::Horizontal), false, false, 0);
    // container.pack_start(&message, true, false, 0);

    Content { container }
  }
}

impl App {
  fn new(app_state: Arc<RwLock<QkAppState>>) -> App {
    // Create a new top level window.
    let window = Window::new(WindowType::Toplevel);
    // Create a the headerbar and it's associated content.
    let header = Header::new();
    // Contains the content within the window.
    let content = Content::new();

    // Set the headerbar as the title bar widget.
    window.set_titlebar(Some(&header.container));
    // Set the title of the window.
    window.set_title("App Name");
    // Set the window manager class.
    window.set_wmclass("app-name", "App name");
    // The icon the app will display.
    Window::set_default_icon_name("iconname");
    // Add the content box into the window.
    window.add(&content.container);

    // Programs what to do when the exit button is used.
    window.connect_delete_event(move |_, _| {
      main_quit();
      Inhibit(false)
    });

    // Return our main application state
    App {
      window,
      header,
      content,
      app_state
    }
  }
}

impl Header {
  fn new() -> Header {
    let (container, btn_cluster) = Header::setup_header_bar();

    // Returns the header and all of it's state
    Header {
      container,
      btn_cluster,
    }
  }

  /// Creates the main header bar container widget.
  fn setup_header_bar() -> (HeaderBar, Button) {
    let container = HeaderBar::new();

    // Sets the text to display in the title section of the header bar.
    container.set_title(Some("Quokka Observer"));
    // Enable the window controls within this headerbar.
    container.set_show_close_button(true);

    // Create the hit and heal buttons.
    let btn_cluster = Button::new_with_label("Cluster View");

    // Add the corresponding style classes to those buttons.
    // btn_cluster.get_style_context().map(|c| c.add_class("destructive-action"));

    // Then add them to the header bar.
    container.pack_start(&btn_cluster);
    (container, btn_cluster)
  }
}