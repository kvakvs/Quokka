use std::ops::Deref;
use std::sync::{Arc, RwLock};

use gtk::BoxExt;
use gtk::prelude::*;
use qk_livesystem::ui::size::Sizef;
use qk_livesystem::ui::layout::TLayout;
use qk_livesystem::ui::draw::TDrawable;

use crate::window::main::app_state::{QkAppState, QkViewMode};
use qk_livesystem::ui::ui_element_state::UiElementState;
use qk_livesystem::ui::styling;

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
      drawing_area.connect_draw(move |da, ctx| {
        Self::drawing_area_draw(da, ctx, &st)
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

  fn drawing_area_draw(da: &gtk::DrawingArea,
                       cr: &cairo::Context,
                       app_state: &RwLock<QkAppState>) -> gtk::Inhibit {
    let app_lock = app_state.read().unwrap();
    match app_lock.view_mode {
      QkViewMode::Cluster => {
        Self::drawing_area_draw_cluster(da, cr, app_lock.deref())
      }
      QkViewMode::Node(pid) => {
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
    cr.scale(1.0, 1.0);
    cr.translate(-app_state.camera_offset.x, -app_state.camera_offset.y);

    // Draw current mode label
    cr.select_font_face("Sans",
                        cairo::FontSlant::Normal,
                        cairo::FontWeight::Normal);
    cr.set_font_size(styling::FONT_SIZE);
    styling::FONT_NORMAL_COLOR.set_source_rgb(cr);
    cr.move_to(8.0, 20.0);
    cr.show_text(&format!("Cluster view, {} nodes", app_state.cluster.nodes.len()));

    // TODO: Layout component for nodes
    app_state.cluster.nodes.iter().for_each(|node| {
      let element_st = UiElementState::Selected;
      node.draw(cr, element_st);
    })
  }
}
