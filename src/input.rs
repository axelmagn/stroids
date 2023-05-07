//! Module for handling player input

use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    math::Vec3Swizzles,
    prelude::{
        Component, EventReader, EventWriter, Events, Input, IntoSystemConfig, KeyCode, MouseButton,
        OnUpdate, Plugin, Query, Res, Transform,
    },
    window::Window,
};

use crate::{app::AppState, collision::Collider};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<InputEvent>();
        app.add_system(system_keyboard_input.in_set(OnUpdate(AppState::InGame)));
        app.add_system(system_click_input);
    }
}

/// A semantic input signal that is controller agnostic
#[derive(Clone, Copy, Debug)]
pub enum InputAction {
    Thrust(f32),
    Turn(f32),
    Shoot,
}

#[derive(Clone, Copy, Debug)]
pub struct InputEvent {
    pub action: InputAction,
    pub state: ButtonState,
    pub scan_code: u32,
}

/// A quick and dirty keyboard input mapping
/// TODO: replace hardcoded keys with configured values
pub fn system_keyboard_input(
    mut evr_keys: EventReader<KeyboardInput>,
    mut evw_input_action: EventWriter<InputEvent>,
) {
    evr_keys.iter().for_each(|ev| {
        let action_opt = ev
            .key_code
            // convert key code to input action
            .and_then(|code| match code {
                KeyCode::W | KeyCode::Up => Some(InputAction::Thrust(1.)),
                KeyCode::S | KeyCode::Down => Some(InputAction::Thrust(-1.)),
                KeyCode::A | KeyCode::Left => Some(InputAction::Turn(1.)),
                KeyCode::D | KeyCode::Right => Some(InputAction::Turn(-1.)),
                KeyCode::Space => Some(InputAction::Shoot),
                _ => None,
            });
        if let Some(action) = action_opt {
            evw_input_action.send(InputEvent {
                action,
                state: ev.state,
                scan_code: ev.scan_code,
            })
        }
    });
}

#[derive(Debug, Default, Component)]
pub struct ClickListener(pub Events<Input<MouseButton>>);

pub fn system_click_input(
    input_mouse: Res<Input<MouseButton>>,
    mut listeners: Query<(&mut ClickListener, &Collider, &Transform)>,
    windows: Query<&Window>,
) {
    if windows.is_empty() {
        return;
    }
    let window = windows.single();
    if window.cursor_position().is_none() {
        return;
    }
    let cursor_pos = window.cursor_position().unwrap();

    listeners
        .iter_mut()
        .for_each(|(mut listener, collider, xform)| {
            listener.0.update();
            let dist = xform.translation.xy().distance(cursor_pos);
            if input_mouse.get_just_pressed().len() > 0 && dist <= collider.radius {
                listener.0.send(input_mouse.clone());
            }
        });
}
