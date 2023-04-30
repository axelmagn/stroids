//! Module for handling player input

use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::{EventReader, EventWriter, KeyCode, Plugin},
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<InputEvent>();
        app.add_system(keyboard_input_system);
    }
}

/// A semantic input signal that is controller agnostic
#[derive(Clone, Copy, Debug)]
pub enum InputAction {
    Thrust(f32),
    Turn(f32),
}

#[derive(Clone, Copy, Debug)]
pub struct InputEvent {
    pub action: InputAction,
    pub state: ButtonState,
    pub scan_code: u32,
}

/// A quick and dirty keyboard input mapping
/// TODO: replace hardcoded keys with configured values
pub fn keyboard_input_system(
    mut evr_keys: EventReader<KeyboardInput>,
    mut evw_input_action: EventWriter<InputEvent>,
) {
    evr_keys.iter().for_each(|ev| {
        ev.key_code
            // convert key code to input action
            .map(|code| match code {
                KeyCode::W => Some(InputAction::Thrust(1.)),
                KeyCode::S => Some(InputAction::Thrust(-1.)),
                KeyCode::A => Some(InputAction::Turn(1.)),
                KeyCode::D => Some(InputAction::Turn(-1.)),
                _ => None,
            })
            .flatten()
            // emit input event
            .map(|action| {
                evw_input_action.send(InputEvent {
                    action,
                    state: ev.state,
                    scan_code: ev.scan_code,
                })
            });
    });
}