use std::sync::{Arc, RwLock};

use gtk::{ContainerExt, WidgetExt, GtkWindowExt};

use crate::window::main;
use crate::window::main::app_state::QkAppState;
use crate::window::main::content::QkMainWindowContent;
use crate::window::main::QkMainWindowHeader;

pub struct QkApp {
  pub window: gtk::ApplicationWindow,
  pub header: QkMainWindowHeader,
  pub content: QkMainWindowContent,
  pub app_state: Arc<RwLock<QkAppState>>,
}

impl QkApp {
  pub fn new(gtk_app: &gtk::Application, app_state: Arc<RwLock<QkAppState>>) -> QkApp {
    // Create a new top level window.
    // let window = gtk::Window::new(gtk::WindowType::Toplevel);
    let window = gtk::ApplicationWindow::new(gtk_app);

    // Create a the headerbar and it's associated content.
    let header = QkMainWindowHeader::new();
    // Contains the content within the window.
    let content = QkMainWindowContent::new(&window, &app_state);

    // Set the headerbar as the title bar widget.
    window.set_titlebar(Some(&header.container));
    // Set the title of the window.
    window.set_title("App Name");
    // Set the window manager class.
    window.set_wmclass("quokka-observer", "QuokkaObserverAppClass");
    // The icon the app will display.
    gtk::Window::set_default_icon_name("iconname");
    // Add the content box into the window.
    // window.add(&content.container);

    // Programs what to do when the exit button is used.
    window.connect_delete_event(move |_, _| {
      gtk::main_quit();
      gtk::Inhibit(false)
    });

    // let draw_window = main::create_drawable1(gtk_app);
    // create_drawable2(gtk_app);

    // Return our main application state
    QkApp {
      window,
      header,
      content,
      app_state,
    }
  }
}
