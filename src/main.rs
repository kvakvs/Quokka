extern crate gtk;
use gtk::*;
use std::process;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
  // Initialize GTK before proceeding.
  if gtk::init().is_err() {
    eprintln!("failed to initialize GTK Application");
    process::exit(1);
  }

  // Set the initial state of our health component. We use an `Arc` so that we can share
  // this value across multiple programmable closures.
  let health = Arc::new(HealthComponent::new(10));

  // Initialize the UI's initial state.
  let app = App::new(&health);

  {
    // Program the Hit button to subtract health.
    let health = health.clone();
    let message = app.content.message.clone();
    let info = app.content.health.clone();
    // app.header.hit.connect_clicked(move |_| {
    //   let new_health = health.subtract(1);
    //   let action = if new_health == 0 { Message::Dead } else { Message::Hit };
    //   message.set_label(MESSAGES[action as usize]);
    //   info.set_label(new_health.to_string().as_str());
    // });
  }

  {
    // Program the Heal button to restore health.
    let health = health.clone();
    let message = app.content.message.clone();
    let info = app.content.health.clone();
    app.header.heal.connect_clicked(move |_| {
      let new_health = health.heal(5);
      //message.set_label(MESSAGES[Message::Heal as usize]);
      info.set_label(new_health.to_string().as_str());
    });
  }

  // Make all the widgets within the UI visible.
  app.window.show_all();

  // Start the GTK main event loop
  gtk::main();
}

pub struct HealthComponent(AtomicUsize);

impl HealthComponent {
  fn new(initial: usize) -> HealthComponent { HealthComponent(AtomicUsize::new(initial)) }

  fn get_health(&self) -> usize { self.0.load(Ordering::SeqCst) }

  fn subtract(&self, value: usize) -> usize {
    let current = self.0.load(Ordering::SeqCst);
    let new = if current < value { 0 } else { current - value };
    self.0.store(new, Ordering::SeqCst);
    new
  }

  fn heal(&self, value: usize) -> usize {
    let original = self.0.fetch_add(value, Ordering::SeqCst);
    original + value
  }
}

pub struct App {
  pub window:  Window,
  pub header:  Header,
  pub content: Content,
}

pub struct Header {
  pub container: HeaderBar,
  pub hit:       Button,
  pub heal:      Button,
}

pub struct Content {
  pub container: Box,
  pub health:    Label,
  pub message:   Label,
}

impl Content {
  fn new(health: &HealthComponent) -> Content {
    // Create a vertical box to store all of it's inner children vertically.
    let container = Box::new(Orientation::Vertical, 0);

    // The health info will be contained within a horizontal box within the vertical box.
    let health_info = Box::new(Orientation::Horizontal, 0);
    let health_label = Label::new(Some("Current Health:"));
    let health = Label::new(Some(health.get_health().to_string().as_str()));

    // Set the horizontal alignments of each of our objects.
    health_info.set_halign(Align::Center);
    health_label.set_halign(Align::Start);
    health.set_halign(Align::Start);


    // Add the health info box's children
    health_info.pack_start(&health_label, false, false, 5);
    health_info.pack_start(&health, true, true, 5);

    // Create a message label that will later be modified by the application, upon
    // performing a hit or heal action.
    let message = Label::new(Some("Hello"));

    // Add everything to our vertical box
    container.pack_start(&health_info, true, false, 0);
    container.pack_start(&Separator::new(Orientation::Horizontal), false, false, 0);
    container.pack_start(&message, true, false, 0);

    Content { container, health, message }
  }
}

impl App {
  fn new(health: &HealthComponent) -> App {
    // Create a new top level window.
    let window = Window::new(WindowType::Toplevel);
    // Create a the headerbar and it's associated content.
    let header = Header::new();
    // Contains the content within the window.
    let content = Content::new(health);

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
    App { window, header, content }
  }
}

impl Header {
  fn new() -> Header {
    // Creates the main header bar container widget.
    let container = HeaderBar::new();

    // Sets the text to display in the title section of the header bar.
    container.set_title(Some("Quokka Observer"));
    // Enable the window controls within this headerbar.
    container.set_show_close_button(true);

    // Create the hit and heal buttons.
    let hit = Button::new_with_label("Hit");
    let heal = Button::new_with_label("Heal");

    // Add the corresponding style classes to those buttons.
    hit.get_style_context().map(|c| c.add_class("destructive-action"));
    heal.get_style_context().map(|c| c.add_class("suggested-action"));

    // THen add them to the header bar.
    container.pack_start(&hit);
    container.pack_end(&heal);

    // Returns the header and all of it's state
    Header { container, hit, heal }
  }
}
