use std::ops::Deref;
use std::sync::{Arc, RwLock};

use gdk::EventType;
use gio::prelude::*;
use gtk::WidgetExt;
use gtk::prelude::*;

use qk_livesystem::ui::draw::TDrawable;
use qk_livesystem::ui::point::Pointf;
use qk_livesystem::ui::styling;
use qk_livesystem::ui::ui_element_state::UiElementState;

use crate::app::{QkApp, QkViewMode};
use crate::ui::controls::Controls;
use crate::window::main::pointer_mode::QkPointerMode;

pub struct QkMainWindowContent {
  pub container: gtk::Box,
  pub draw_area: gtk::DrawingArea,
  // pub health: gtk::Label,
  // pub message: gtk::Label,
}

impl QkMainWindowContent {
  pub fn new(window: &gtk::ApplicationWindow,
             app_state: &Arc<RwLock<QkApp>>) -> QkMainWindowContent {
    // Create a vertical box to store all of it's inner children vertically.
    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);

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

    QkMainWindowContent {
      container,
      draw_area: drawing_area,
    }
  }

  fn handle_event(_this: &gtk::ApplicationWindow,
                  ev: &gdk::Event,
                  app_state: &RwLock<QkApp>) -> gtk::Inhibit {
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
                         app_state: &RwLock<QkApp>) -> gtk::Inhibit {
    let pos = ev.get_coords().unwrap();
    let button = ev.get_button().unwrap();
    println!("Button press {} at {:?}", button, pos);

    // TODO: hit test displayed objects

    Inhibit(true)
  }

  fn handle_button_release(_this: &gtk::ApplicationWindow,
                           ev: &gdk::Event,
                           app_state: &RwLock<QkApp>) -> gtk::Inhibit {
    let pointer_mode = QkApp::read_with(app_state,
                                        |r| { r.pointer_mode });
    match pointer_mode {
      QkPointerMode::Normal => {} // no action
      QkPointerMode::Pan(_) => {
        // end panning
        QkApp::modify_with(app_state,
                           |w| { w.pointer_mode = QkPointerMode::Normal })
      }
    }
    Inhibit(true)
  }

  fn handle_motion(this: &gtk::ApplicationWindow,
                   ev: &gdk::Event,
                   app_state: &RwLock<QkApp>) -> gtk::Inhibit {
    let pointer_mode = QkApp::read_with(app_state,
                                        |r| { r.pointer_mode });
    match pointer_mode {
      QkPointerMode::Normal => {} // no action
      QkPointerMode::Pan(_) => {
        let pos = ev.get_coords().unwrap();
        println!("Right mouse drag {:?}", pos);
      }
    }

    Inhibit(true)
  }

  fn drawing_area_draw(da: &gtk::DrawingArea,
                       cr: &cairo::Context,
                       app_state: &RwLock<QkApp>) -> gtk::Inhibit {
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
                               app_state: &QkApp) {
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
