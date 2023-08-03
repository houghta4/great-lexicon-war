use bevy::prelude::*;

pub const BACKGROUND_COLOR: BackgroundColor =
    BackgroundColor(Color::rgba(0.984375, 0.95703125, 0.89453125, 0.5));

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.99609375, 0.64453125, 0.);

pub fn get_pause_menu_style() -> Style {
    Style {
        position_type: PositionType::Absolute, // Needed to display separately from HUD.
        display: Display::Flex,                // Hidden by Default
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Percent(100.),
        height: Val::Percent(100.0),
        ..default()
    }
}

pub fn get_pause_menu_container_style() -> Style {
    Style {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(400.0),
        height: Val::Px(400.0),
        row_gap: Val::Px(8.0),
        column_gap: Val::Px(8.0),
        ..default()
    }
}

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/propaganda/propaganda.ttf"),
        font_size: 32.,
        color: Color::BLACK,
    }
}

pub fn get_button_style() -> Style {
    Style {
        width: Val::Px(200.0),
        height: Val::Px(80.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/propaganda/propaganda.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
    }
}
