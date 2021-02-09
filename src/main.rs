extern crate qk_livesystem;

use crate::app::QkApp;
use imgui::*;
use qk_livesystem::ui::ui_element_state::UiElementState;
use qk_livesystem::ui::draw::TDrawable;

mod window;
mod ui;
mod app;

pub const WINDOW_TITLE: &'static str = "Quokka Observer";

fn main() {
  let mut state = QkApp::new();
  let system = crate::ui::startup::init(WINDOW_TITLE);

  system.main_loop(move |run, ui| {
    quokka_menubar(ui, run, &mut state);
    quokka_cluster_view(ui, &mut state)
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

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let draw_list = ui.get_window_draw_list();
        // Will draw channel 0 first, then channel 1, whatever the order of
        // the calls in the code.
        //
        // Here, we draw a red line on channel 1 then a white circle on
        // channel 0. As a result, the red line will always appear on top of
        // the white circle.
        draw_list.channels_split(2, |channels| {
          let canvas_pos = ui.cursor_screen_pos();
          channels.set_current(1);
          app.cluster.nodes.iter().for_each(|n| {
            n.draw(canvas_pos.into(), &draw_list, UiElementState::NotSelected);
          });

          // Draw under

          // channels.set_current(0);
          // let center = [canvas_pos[0] + RADIUS, canvas_pos[1] + RADIUS];
          // draw_list
          //     .add_circle(center, RADIUS, WHITE)
          //     .thickness(10.0)
          //     .num_segments(50)
          //     .build();
        });
      });
}
