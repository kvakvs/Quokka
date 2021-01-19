use gtk::prelude::*;
use gtk::ApplicationWindow;

const TITLE: &str = "Quokka Observer";

pub fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);

    set_title(window.clone());
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    let button = gtk::Button::with_label("Click me!");
    button.connect_clicked(|_b| {
        println!("Clicked!");
    });

    window.add(&button);

    window.show_all();
}

/// Set the title.
// TODO: set sub-titles too, using (TITLE + "-" + s)
pub fn set_title(w: ApplicationWindow) {
    w.set_title(TITLE);
}
