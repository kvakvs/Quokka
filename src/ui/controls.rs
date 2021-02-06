use gdk::EventType;

pub struct Controls {}

impl Controls {
  pub fn is_pan_start_event(ev: &gdk::Event) -> bool {
    ev.get_event_type() == EventType::ButtonPress && ev.get_button() == Some(3)
  }
}
