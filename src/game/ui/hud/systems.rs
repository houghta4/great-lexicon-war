use crate::game::player::components::Player;
use crate::game::resources::{RandomWord, WordBank};
use crate::game::word_match::components::Word;
use crate::game::WordComplexity;
use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::ui::Val;
use bevy::utils::default;

use super::components::{
    HealText, PlayerAmmoText, PlayerHealthBar, PlayerHealthPack, PlayerHealthText, ReloadText,
};

/// Update player's health in HUD
///
/// Change colors depending on percentage
pub fn update_health(
    mut text_q: Query<&mut Text, With<PlayerHealthText>>,
    player_q: Query<&Player, Changed<Player>>,
) {
    if let Ok(player) = player_q.get_single() {
        if let Ok(mut health_bundle) = text_q.get_single_mut() {
            let mut health = &mut health_bundle.sections[0];
            health.value = format!("{}", player.health);
            match health {
                ref mut h if player.health > 75.0 => h.style.color = Color::GREEN,
                ref mut h if player.health > 50.0 => h.style.color = Color::YELLOW,
                ref mut h if player.health > 25.0 => h.style.color = Color::ORANGE_RED,
                _ => health.style.color = Color::RED,
            }
        }
    }
}

/// Update player's ammo count in HUD
///
/// Change colors depending on percentage
pub fn update_ammo(
    mut text_q: Query<&mut Text, With<PlayerAmmoText>>,
    player_q: Query<&Player, Changed<Player>>,
) {
    if let Ok(player) = player_q.get_single() {
        if let Ok(mut ammo_bundle) = text_q.get_single_mut() {
            ammo_bundle.sections[2].value = format!("{}", player.ammo.1);
            let ammo = &mut ammo_bundle.sections[0];
            ammo.value = format!("{}", player.ammo.0);
            match player.ammo.0 as f32 / player.ammo.1 as f32 {
                a if a > 0.75 => ammo.style.color = Color::WHITE,
                a if a > 0.5 => ammo.style.color = Color::YELLOW,
                a if a > 0.25 => ammo.style.color = Color::ORANGE_RED,
                _ => ammo.style.color = Color::RED,
            }
        }
    }
}

/// Reset `WordTarget::Reload` after a `PlayerReloadEvent` is fired
pub fn update_reload_word(
    mut word_bank: ResMut<WordBank>,
    word_q: Query<&Word, (With<Word>, Without<ReloadText>)>,
    mut reload_q: Query<(&mut Text, &mut Word), With<ReloadText>>,
) {
    if let Ok((mut text, mut word)) = reload_q.get_single_mut() {
        let w = word_bank.get_word(WordComplexity::Easy, &word_q);
        text.sections[1].value = w.to_owned();
        word.1 = w.to_owned();
    }
}

/// Change the HUD health bar's width and color depending on the health of the player
pub fn update_health_bar(
    mut style_q: Query<(&mut Style, &mut BackgroundColor), With<PlayerHealthBar>>,
    player_q: Query<&Player, With<Player>>,
) {
    if let Ok(player) = player_q.get_single() {
        if let Ok((mut style, mut bg)) = style_q.get_single_mut() {
            if let Val::Percent(p) = style.width {
                // .4 seems to be around the sweet spot
                // Any larger and it is too fast. Any smaller and it is choppy
                let perc = 0.4;
                if p <= 0. {
                    // set background to none here so decreasing animation can fully complete
                    *bg = BackgroundColor(Color::NONE);
                } else if p > player.health {
                    style.width = style
                        .width
                        .try_sub(Val::Percent(perc))
                        .unwrap_or(style.width);
                } else if p + perc < player.health {
                    style.width = style
                        .width
                        .try_add(Val::Percent(perc))
                        .unwrap_or(style.width);
                }
            }

            match player.health {
                h if h > 75.0 => bg.0 = Color::GREEN,
                h if h > 50.0 => bg.0 = Color::YELLOW,
                h if h > 25.0 => bg.0 = Color::ORANGE_RED,
                h if h > 0.0 => bg.0 = Color::RED,
                _ => (),
            }
        }
    }
}

/// Change the amount of health packs displayed in the HUD based on how many the player has left
pub fn update_health_packs(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_q: Query<&Player, Changed<Player>>,
    container_q: Query<(Entity, &Children), With<PlayerHealthPack>>,
) {
    if let Ok(player) = player_q.get_single() {
        if let Ok((container, children)) = container_q.get_single() {
            if children.len() != player.health_packs {
                // TODO no way to remove specific child?
                commands.entity(container).despawn_descendants();
                commands.entity(container).with_children(|builder| {
                    for _ in 0..player.health_packs {
                        builder.spawn(get_health_bundle(&asset_server));
                    }
                });
            }
        }
    }
}

/// Reset `WordTarget::Heal` after the player heals
pub fn update_heal_word(
    mut word_bank: ResMut<WordBank>,
    word_q: Query<&Word, (With<Word>, Without<HealText>)>,
    mut heal_q: Query<(&mut Text, &mut Word), With<HealText>>,
    player_q: Query<&Player, With<Player>>,
) {
    if let Ok(player) = player_q.get_single() {
        if let Ok((mut text, mut word)) = heal_q.get_single_mut() {
            let w = word_bank.get_word(WordComplexity::Easy, &word_q);
            text.sections[1].value = w.to_owned();
            word.1 = w.to_owned();

            // TODO: should there be an indication if empty?
            if player.health_packs == 0 {
                text.sections[1].style.color = Color::RED;
            }
        }
    }
}

/// Return ImageBundle of health pack image
pub fn get_health_bundle(asset_server: &Res<AssetServer>) -> ImageBundle {
    ImageBundle {
        image: UiImage::new(asset_server.load("sprites/health.png")),
        style: Style {
            margin: UiRect::horizontal(Val::Px(10.)),
            ..default()
        },
        ..default()
    }
}
