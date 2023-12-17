use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use crate::game::input::components::InputText;
use crate::game::input::resource::BackspaceTimer;
use crate::game::input::events::InputEvent;

/**
    Listens for keyboard key inputs (a-z and A-Z) and appends them onto the input text
**/
pub fn listen_received_character_events(
    mut events: EventReader<ReceivedCharacter>,
    mut input_writer: EventWriter<InputEvent>,
    mut edit_text: Query<&mut Text, With<InputText>>,
) {
    for event in events.iter() {
        if (event.char >= 'a' && event.char <= 'z') || (event.char >= 'A' && event.char <= 'Z') {
            edit_text.single_mut().sections[0].value.push(event.char);
            input_writer.send(InputEvent);
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
    mut timer: ResMut<BackspaceTimer>,
    mut input_writer: EventWriter<InputEvent>
) {
    for event in events.iter() {
        match event.key_code {
            Some(KeyCode::Return) => {
                edit_text.single_mut().sections[0].value = "".to_string();
                input_writer.send(InputEvent);
            }
            _ => continue,
        }
    }

    if input.pressed(KeyCode::Back) && timer.0.tick(time.delta()).just_finished() {
        input_writer.send(InputEvent);
        edit_text.single_mut().sections[0].value.pop();
    }
}
