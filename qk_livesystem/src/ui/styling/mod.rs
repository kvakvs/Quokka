use style_color::StyleColor;

pub mod style_color;
pub mod line_style;
pub mod font_style;

/// Default selected color for lines and text
pub const DEFAULT_SELECTED_COLOR: StyleColor = StyleColor::RGB(0.2, 0.4, 0.9);

/// Editor background color
pub const BACKGROUND_COLOR: StyleColor = StyleColor::RGB(1.0, 1.0, 1.0);
