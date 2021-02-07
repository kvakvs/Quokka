use gtk::{ButtonExt, HeaderBarExt};

pub struct QkMainWindowHeader {
  pub container: gtk::HeaderBar,
  pub btn_cluster: gtk::Button, // Cluster view
}

impl QkMainWindowHeader {
  pub fn new() -> QkMainWindowHeader {
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
