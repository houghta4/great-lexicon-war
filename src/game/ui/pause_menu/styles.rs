use bevy::prelude::*;

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

pub fn get_button_style(width: f32, height: f32) -> Style {
    Style {
        width: Val::Px(width),
        height: Val::Px(height),
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
