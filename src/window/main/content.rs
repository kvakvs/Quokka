use gtk::BoxExt;
use gtk::prelude::*;

pub struct QkMainWindowContent {
  pub container: gtk::Box,
  // pub health: gtk::Label,
  // pub message: gtk::Label,
}

impl QkMainWindowContent {
  pub fn new(window: &gtk::ApplicationWindow) -> QkMainWindowContent {
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
    // let message = gtk::Label::new(Some("Now looking at: Cluster view"));
    let drawing_area = std::boxed::Box::new(gtk::DrawingArea::new)();
    window.add(&drawing_area);
    drawing_area.connect_draw(Self::drawing_area_draw);
    container.pack_start(&drawing_area, true, false, 0);

    // Add everything to our vertical box
    // container.pack_start(&health_info, true, false, 0);
    // container.pack_start(&Separator::new(gtk::Orientation::Horizontal), false, false, 0);
    // container.pack_start(&message, true, false, 0);

    QkMainWindowContent { container }
  }

  fn drawing_area_draw(_da: &gtk::DrawingArea, cr: &cairo::Context) -> gtk::Inhibit {
    cr.scale(500f64, 500f64);

    cr.select_font_face("Sans",
                        cairo::FontSlant::Normal,
                        cairo::FontWeight::Normal);
    cr.set_font_size(0.35);

    cr.move_to(0.04, 0.53);
    cr.show_text("Hello");

    gtk::Inhibit(false)
  }
}
