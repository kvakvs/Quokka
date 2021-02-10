use std::sync::{Arc, RwLock};
use qk_data::data_stream::eflame_log::EflameLogStream;
use qk_livesystem::beam_cluster::BeamCluster;
use qk_livesystem::ui::point::Pointf;
use qk_term::atom::Atom;
use imgui::*;

use crate::window::main::node_selection::QkNodeSelection;
use crate::window::main::pointer_mode::QkPointerMode;
use crate::window::main::process_selection::QkProcessSelection;
use qk_livesystem::ui::ui_element_state::UiElementState;
use qk_livesystem::ui::draw::TDrawable;

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum QkViewMode {
  Cluster,
  Node(Atom),
  NodeCode(Atom),
}

impl QkViewMode {
  pub fn get_node(self) -> Option<Atom> {
    match self {
      QkViewMode::Cluster => { None }
      QkViewMode::Node(a) => { Some(a) }
      QkViewMode::NodeCode(b) => { Some(b) }
    }
  }
}

/// Stores application global state: That is current opened project, view mode etc.
pub struct QkApp {
  /// Whether hints should be visible. TODO: Save in config
  show_help: bool,

  // TODO: this below belongs to the current project, when projects are introduced
  pub view_mode: QkViewMode,
  pub camera_zoom: f64,
  pub camera_offset: Pointf,
  pub cluster: BeamCluster,

  /// Whether none, one or multiple NODES in the CLUSTER view are selected
  pub node_selection: QkNodeSelection,

  /// Whether none, one or multiple PROCESSES in the NODE view are selected
  pub process_selection: QkProcessSelection,

  /// Whether mouse is busy doing something special like dragging
  pub pointer_mode: QkPointerMode,
}

impl QkApp {
  pub fn new() -> Self {
    Self {
      show_help: true,
      view_mode: QkViewMode::Cluster,
      camera_zoom: 1.0,
      camera_offset: Pointf::new(0.0, 0.0),
      cluster: Default::default(),
      node_selection: Default::default(),
      process_selection: Default::default(),
      pointer_mode: Default::default(),
    }
  }

  #[allow(dead_code)]
  pub fn load(&mut self) {
    let ef_log = std::boxed::Box::new(EflameLogStream::new("eflame_log.txt").unwrap());
    // ef_log.lines.iter().for_each(|line| { println!("{:?}", line) });

    // Representation of the live system as we know it
    self.cluster.load_data_stream(ef_log);
  }

  // pub fn set_view_mode(this_rw: &RwLock<QkAppState>, view_mode: QkViewMode) {
  //   let mut this = this_rw.write().unwrap();
  //
  //   this.view_mode = view_mode;
  //   // TODO: Redraw the window
  //
  //   drop(this);
  // }

  pub fn read_with<T, TFun>(state_rwlock: &RwLock<Self>, func: TFun) -> T
    where TFun: Fn(&Self) -> T {
    let state = state_rwlock.read().unwrap();
    let result = func(&state);
    drop(state);
    result
  }

  pub fn modify_with<TFun>(state_rwlock: &RwLock<Self>, mut_func: TFun)
    where TFun: Fn(&mut Self) {
    // Type for TFun fn(state: &mut Self) -> ()
    let mut state = state_rwlock.write().unwrap();
    mut_func(&mut state);
    drop(state);
  }

  /// Attempt to hit one node with mouse_pos, return QkNodeSelection as a result (one or none).
  pub fn try_select_one_node(&mut self, mouse_pos: &Pointf) -> QkNodeSelection {
    if let Some(node) = self.cluster.nodes.iter().find(|node| {
      node.is_mouse_hit(mouse_pos)
    }) {
      QkNodeSelection::One(node.name)
    } else {
      QkNodeSelection::None
    }
  }

  /// Draw a window "BrowseWindow" (it will retain size of previous BrowseWindow) for when we're
  /// viewing a BEAM cluster (all connected nodes that we know).
  pub fn cluster_view(&mut self, ui: &mut Ui) {
    imgui::Window::new(imgui::im_str!("Cluster View###BrowseWindow"))
        .size([800.0, 500.0], Condition::FirstUseEver)
        .resizable(true)
        .build(ui, || {
          if self.show_help {
            ui.text(im_str!("Double click nodes to look inside"));
          }

          let canvas_pos = Pointf::from(ui.cursor_screen_pos());
          let mouse_pos = Pointf::from(ui.io().mouse_pos) - canvas_pos;

          if ui.is_mouse_double_clicked(MouseButton::Left) {
            // Double Clicking a node, also enter the node inner view
            self.node_selection = self.try_select_one_node(&mouse_pos);
            if let QkNodeSelection::One(node_name) = self.node_selection {
              self.view_mode = QkViewMode::Node(node_name)
            }
          } else if ui.is_mouse_clicked(MouseButton::Left) {
            // A click can find 0 or 1 object
            self.node_selection = self.try_select_one_node(&mouse_pos);
            println!("Selected: {:?}", self.node_selection);
          }
          // const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
          // const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

          let draw_list = ui.get_window_draw_list();
          // Will draw channel 0 first, then channel 1, whatever the order of
          // the calls in the code.
          //
          // Here, we draw a red line on channel 1 then a white circle on
          // channel 0. As a result, the red line will always appear on top of
          // the white circle.
          draw_list.channels_split(2, |channels| {
            channels.set_current(1);

            self.cluster.nodes.iter().for_each(|n| {
              let ui_element_state = match &self.node_selection {
                QkNodeSelection::None => { UiElementState::NotSelected }
                QkNodeSelection::One(selected_node) => {
                  if *selected_node == n.name {
                    UiElementState::Selected
                  } else {
                    UiElementState::NotSelected
                  }
                }
                QkNodeSelection::Multiple(names) => {
                  if names.contains(&n.name) {
                    UiElementState::Selected
                  } else {
                    UiElementState::NotSelected
                  }
                }
              };
              n.draw(canvas_pos, &draw_list, ui_element_state);
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

  /// Draw a window "BrowseWindow" (it will retain size of previous BrowseWindow) for when we're
  /// viewing a BEAM node with the name stored in self.view_mode.
  pub fn node_view(&mut self, ui: &mut Ui) {
    imgui::Window::new(imgui::im_str!("Node View###BrowseWindow"))
        .size([800.0, 500.0], Condition::FirstUseEver)
        .resizable(true)
        .build(ui, || {
          if self.show_help {
            // ui.text(im_str!("Double click nodes to look inside"));
          }

          if ui.button(im_str!("Return to Cluster"), [128.0, 0.0]) {
            self.view_mode = QkViewMode::Cluster
          }
          ui.same_line(140.0);
          if ui.button(im_str!("View Code"), [0.0, 0.0]) {
            let curr_node = self.view_mode.get_node();
            self.view_mode = QkViewMode::NodeCode(curr_node.unwrap());
          }

          let canvas_pos = Pointf::from(ui.cursor_screen_pos());
          // let mouse_pos = Pointf::from(ui.io().mouse_pos) - canvas_pos;

          // if ui.is_mouse_clicked(MouseButton::Left) {
          //   // A click can find 0 or 1 object
          //   self.node_selection = self.try_select_one_node(&mouse_pos);
          // } else if ui.is_mouse_double_clicked(MouseButton::Left) {
          //   // Double Clicking a node, also enter the node inner view
          //   self.node_selection = self.try_select_one_node(&mouse_pos);
          //   if let QkNodeSelection::One(node_name) = self.node_selection {
          //     self.view_mode = QkViewMode::Node(node_name)
          //   }
          // }

          let draw_list = ui.get_window_draw_list();
          // Will draw channel 0 first, then channel 1, whatever the order of
          // the calls in the code.
          //
          // Here, we draw a red line on channel 1 then a white circle on
          // channel 0. As a result, the red line will always appear on top of
          // the white circle.
          draw_list.channels_split(2, |channels| {
            channels.set_current(1);

            // self.cluster.nodes.iter().for_each(|n| {
            //   let ui_element_state = match &self.node_selection {
            //     QkNodeSelection::None => { UiElementState::NotSelected }
            //     QkNodeSelection::One(selected_node) => {
            //       if *selected_node == n.name {
            //         UiElementState::Selected
            //       } else {
            //         UiElementState::NotSelected
            //       }
            //     }
            //     QkNodeSelection::Multiple(names) => {
            //       if names.contains(&n.name) {
            //         UiElementState::Selected
            //       } else {
            //         UiElementState::NotSelected
            //       }
            //     }
            //   };
            //   n.draw(canvas_pos, &draw_list, ui_element_state);
            // });

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

  /// Draw a window "BrowseWindow" (it will retain size of previous BrowseWindow) for when we're
  /// viewing a modules, apps and functions of a BEAM node with the name stored in self.view_mode.
  pub fn node_code_view(&mut self, ui: &mut Ui) {
    imgui::Window::new(imgui::im_str!("Node Code View###BrowseWindow"))
        .size([800.0, 500.0], Condition::FirstUseEver)
        .resizable(true)
        .build(ui, || {
          if self.show_help {
            ui.text(im_str!("Double click Modules to show contents. Code menu above shows available relations"));
          }

          if ui.button(im_str!("Return to Node"), [0.0, 0.0]) {
            let curr_node = self.view_mode.get_node();
            self.view_mode = QkViewMode::Node(curr_node.unwrap());
          }

          let canvas_pos = Pointf::from(ui.cursor_screen_pos());
          // let mouse_pos = Pointf::from(ui.io().mouse_pos) - canvas_pos;

          // if ui.is_mouse_clicked(MouseButton::Left) {
          //   // A click can find 0 or 1 object
          //   self.node_selection = self.try_select_one_node(&mouse_pos);
          // } else if ui.is_mouse_double_clicked(MouseButton::Left) {
          //   // Double Clicking a node, also enter the node inner view
          //   self.node_selection = self.try_select_one_node(&mouse_pos);
          //   if let QkNodeSelection::One(node_name) = self.node_selection {
          //     self.view_mode = QkViewMode::Node(node_name)
          //   }
          // }

          let draw_list = ui.get_window_draw_list();
          // Will draw channel 0 first, then channel 1, whatever the order of
          // the calls in the code.
          //
          // Here, we draw a red line on channel 1 then a white circle on
          // channel 0. As a result, the red line will always appear on top of
          // the white circle.
          draw_list.channels_split(2, |channels| {
            channels.set_current(1);

            // self.cluster.nodes.iter().for_each(|n| {
            //   let ui_element_state = match &self.node_selection {
            //     QkNodeSelection::None => { UiElementState::NotSelected }
            //     QkNodeSelection::One(selected_node) => {
            //       if *selected_node == n.name {
            //         UiElementState::Selected
            //       } else {
            //         UiElementState::NotSelected
            //       }
            //     }
            //     QkNodeSelection::Multiple(names) => {
            //       if names.contains(&n.name) {
            //         UiElementState::Selected
            //       } else {
            //         UiElementState::NotSelected
            //       }
            //     }
            //   };
            //   n.draw(canvas_pos, &draw_list, ui_element_state);
            // });

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
}
