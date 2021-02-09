extern crate qk_livesystem;

use crate::app::QkApp;
use imgui::*;

mod window;
mod ui;
mod app;

pub const WINDOW_TITLE: &'static str = "Quokka Observer";

fn main() {
  let mut state = QkApp::new();
  let system = crate::ui::startup::init(WINDOW_TITLE);

  system.main_loop(move |run, ui| {
    quokka_menubar(ui, &mut state);
    quokka_cluster_view(ui, &mut state)
  });
}

fn quokka_menubar(ui: &mut Ui, app: &mut QkApp) {
  ui.main_menu_bar(|| {
    //---------------------
    // Test Menu
    //---------------------
    ui.menu(im_str!("Test"), true, || {
      if MenuItem::new(im_str!("Load Eflame2 Test"))
          .shortcut(im_str!("Alt+1"))
          .build(ui) {
        app.load()
      }
    })
  });
}

fn quokka_cluster_view(ui: &mut Ui, app: &mut QkApp) {
  imgui::Window::new(imgui::im_str!("Cluster View"))
      .size([250.0, 500.0], Condition::Always) // was Condition::FirstUseEver
      .build(ui, || {
        // ui.text(im_str!("Cluster View"));
        // ui.button(im_str!("Cluster"), [0.0, 0.0]);
        // ui.button(im_str!("Node"), [0.0, 0.0]);
        // ui.separator();

        // let mouse_pos = ui.io().mouse_pos;
        // ui.text(format!(
        //   "Mouse Position: ({:.1},{:.1})",
        //   mouse_pos[0], mouse_pos[1]
        // ));
      });
}
