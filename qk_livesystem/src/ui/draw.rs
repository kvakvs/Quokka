pub trait TDrawable {
  fn draw(&self, cr: &cairo::Context);
}