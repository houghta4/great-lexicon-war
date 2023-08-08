use bevy::prelude::{BackgroundColor, Color};

pub const BACKGROUND_COLOR: BackgroundColor =
    BackgroundColor(Color::rgba(0.984375, 0.95703125, 0.89453125, 0.5));
pub const SECTION_BACKGROUND_COLOR: BackgroundColor = BackgroundColor(NORMAL_BUTTON);

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.99609375, 0.64453125, 0.);