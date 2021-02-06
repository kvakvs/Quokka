use std::ops::Deref;
use std::sync::{Arc, RwLock};

use gtk::prelude::*;
use gtk::{WidgetExt};

use qk_livesystem::ui::draw::TDrawable;
use qk_livesystem::ui::styling;
use qk_livesystem::ui::ui_element_state::UiElementState;

use crate::window::main::app_state::{QkAppState, QkViewMode};
use gdk::{EventType};
use crate::window::main::pointer_mode::QkPointerMode;
use qk_livesystem::ui::point::Pointf;
use crate::ui::controls::Controls;

pub struct QkMainWindowContent {
  pub container: gtk::Box,
  // pub health: gtk::Label,
  // pub message: gtk::Label,
}

impl QkMainWindowContent {
  pub fn new(window: &gtk::ApplicationWindow,
             app_state: &Arc<RwLock<QkAppState>>) -> QkMainWindowContent {
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
    {
      let st = app_state.clone();
      drawing_area.connect_draw(move |this, ctx| {
        Self::drawing_area_draw(this, ctx, &st)
      });
    }
    {
      let st = app_state.clone();
      window.connect_event(move |this, ev| {
        Self::handle_event(this, ev, &st)
      });
    }

    // container.pack_start(&drawing_area, true, false, 0);
    window.add(&drawing_area);

    // Add everything to our vertical box
    // container.pack_start(&health_info, true, false, 0);
    // container.pack_start(&Separator::new(gtk::Orientation::Horizontal), false, false, 0);
    // container.pack_start(&message, true, false, 0);

    QkMainWindowContent { container }
  }

  fn handle_event(_this: &gtk::ApplicationWindow,
                  ev: &gdk::Event,
                  app_state: &RwLock<QkAppState>) -> gtk::Inhibit {
    if Controls::is_pan_start_event(ev) {
      // Right mouse click and drag - enter the document panning mode
      let pos = ev.get_coords().unwrap();
      let mut state = app_state.write().unwrap();
      state.pointer_mode = QkPointerMode::Pan(Pointf::new(pos.0, pos.1));
      drop(state);
    }

    match ev.get_event_type() {
      EventType::MotionNotify => { Self::handle_motion(_this, ev, app_state) }
      EventType::ButtonPress => { Self::handle_button_press(_this, ev, app_state) }
      // EventType::DoubleButtonPress => {}
      // EventType::TripleButtonPress => {}
      EventType::ButtonRelease => { Self::handle_button_release(_this, ev, app_state) }
      // EventType::KeyPress => {}
      // EventType::KeyRelease => {}
      // EventType::EnterNotify => {}
      // EventType::LeaveNotify => {}
      // EventType::FocusChange => {}
      // EventType::Configure => {}
      // EventType::SelectionClear => {}
      // EventType::SelectionRequest => {}
      // EventType::SelectionNotify => {}
      // EventType::DragEnter => {}
      // EventType::DragLeave => {}
      // EventType::DragMotion => {}
      // EventType::DragStatus => {}
      // EventType::DropStart => {}
      // EventType::DropFinished => {}
      // EventType::Scroll => {}
      // EventType::WindowState => {}
      // EventType::Setting => {}
      // EventType::GrabBroken => {}
      _ => { Inhibit(false) }
    }
  }

  fn handle_button_press(_this: &gtk::ApplicationWindow,
                         ev: &gdk::Event,
                         app_state: &RwLock<QkAppState>) -> gtk::Inhibit {
    let pos = ev.get_coords().unwrap();
    let button = ev.get_button().unwrap();
    println!("Button press {} at {:?}", button, pos);

    // TODO: hit test displayed objects

    Inhibit(true)
  }

  fn handle_button_release(_this: &gtk::ApplicationWindow,
                           ev: &gdk::Event,
                           app_state: &RwLock<QkAppState>) -> gtk::Inhibit {
    Inhibit(true)
  }

  fn handle_motion(_this: &gtk::ApplicationWindow,
                   ev: &gdk::Event,
                   app_state: &RwLock<QkAppState>) -> gtk::Inhibit {
    let button = ev.get_button().unwrap();

    if button == 3 { // Right Mouse
      let state = app_state.read().unwrap();
      println!("Right mouse drag");
      drop(state);
    }

    Inhibit(true)
  }

  fn drawing_area_draw(da: &gtk::DrawingArea,
                       cr: &cairo::Context,
                       app_state: &RwLock<QkAppState>) -> gtk::Inhibit {
    let app_lock = app_state.read().unwrap();
    match app_lock.view_mode {
      QkViewMode::Cluster => {
        Self::drawing_area_draw_cluster(da, cr, app_lock.deref())
      }
      QkViewMode::Node(_pid) => {
        // Self::drawing_area_draw_node(pid, da, cr, app_lock.deref())
        panic!("Can't draw node view yet")
      }
    }
    drop(app_lock);
    gtk::Inhibit(false)
  }

  fn drawing_area_draw_cluster(_da: &gtk::DrawingArea,
                               cr: &cairo::Context,
                               app_state: &QkAppState) {
    styling::BACKGROUND_COLOR.clear_with_color(cr);

    // Draw current mode label
    styling::font_style::apply_info_overlay_font_style(cr);
    cr.move_to(8.0, 20.0);
    cr.show_text(&format!("Cluster view, {} nodes", app_state.cluster.nodes.len()));

    // Reposition camera and set scale
    cr.scale(1.0, 1.0);
    cr.translate(-app_state.camera_offset.x, -app_state.camera_offset.y);

    // TODO: Layout component for nodes
    app_state.cluster.nodes.iter().for_each(|node| {
      let element_st = UiElementState::Selected;
      node.draw(cr, element_st);
    })
  }
}
