extern crate qk_livesystem;

use crate::app::{QkApp, QkViewMode};
use imgui::*;
// use qk_livesystem::ui::ui_element_state::UiElementState;
use qk_livesystem::ui::draw::TDrawable;
use qk_livesystem::ui::point::Pointf;
use qk_livesystem::beam_node::BeamNode;
use crate::window::main::node_selection::QkNodeSelection;

mod window;
mod ui;
mod app;

pub const WINDOW_TITLE: &'static str = "Quokka Observer";

fn main() {
  let mut state = QkApp::new();
  let system = crate::ui::startup::init(WINDOW_TITLE);

  system.main_loop(move |run, ui| {
    quokka_menubar(ui, run, &mut state);
    match state.view_mode {
      QkViewMode::Cluster => { state.cluster_view(ui) }
      QkViewMode::Node(_) => { state.node_view(ui) }
    }
    // state.cluster_view(ui);
    // state.node_view(ui);
  });
}

fn quokka_menubar(ui: &mut Ui, run: &mut bool, app: &mut QkApp) {
  ui.main_menu_bar(|| {
    //---------------------
    // Test Menu
    //---------------------
    ui.menu(im_str!("Quokka"), true, || {
      if MenuItem::new(im_str!("Exit")).build(ui) {
        *run = false;
      }
    });
    ui.menu(im_str!("Test"), true, || {
      if MenuItem::new(im_str!("Load Eflame2 Test"))
          .shortcut(im_str!("Alt+1"))
          .build(ui) {
        app.load()
      }
    })
  });
}
