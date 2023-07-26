use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::game::input::components::InputText;
use crate::game::input::resource::BackspaceTimer;


/**
    Setup for the text input. Spawns a text field for input display
**/
pub fn setup_text_input(
    mut commands: Commands,
    win_q: Query<&Window, With<PrimaryWindow>>) {

    // normally can't just unwrap, but this is guaranteed to exist from Bevy
    let win = win_q.get_single().unwrap();

    commands.spawn((Text2dBundle {
        text: Text::from_section(
            "Type here".to_string(),
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            }
        ),
        transform: Transform::from_xyz(win.width() / 2.0, win.height() / 2.0, 0.0),
        ..default()
    }, InputText));
}

/**
    Listens for keyboard key inputs (a-z and A-Z) and appends them onto the input text
**/
pub fn listen_received_character_events(
    mut events: EventReader<ReceivedCharacter>,
    mut edit_text: Query<&mut Text, With<InputText>>
) {
    for event in events.iter() {
        if (event.char >= 'a' && event.char <= 'z') || (event.char >= 'A' && event.char <= 'Z') {
            edit_text.single_mut().sections[0].value.push(event.char);
        }
    }
}

/**
    Lists for keyboard Enter and Backspace keys.
    Enter: temporary "submit" button, clears input text field
    Backspace: pop one character off the input text field, limited by the BackspaceTimer
**/
pub fn listen_keyboard_input_events(
    mut events: EventReader<KeyboardInput>,
    mut edit_text: Query<&mut Text, With<InputText>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut timer: ResMut<BackspaceTimer>
) {

    for event in events.iter() {
        match event.key_code {
            Some(KeyCode::Return) => {
                edit_text.single_mut().sections[0].value = "".to_string();
            }
            _ => continue,
        }
    }

    if input.pressed(KeyCode::Back) {
        if timer.0.tick(time.delta()).just_finished() {
            edit_text.single_mut().sections[0].value.pop();
        }
    }
}

