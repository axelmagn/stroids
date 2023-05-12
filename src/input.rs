//! Module for handling player input

use std::cmp::Ordering;

use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    math::Vec3Swizzles,
    prelude::{
        Camera, Component, EventReader, EventWriter, Events, GlobalTransform, Input,
        IntoSystemConfig, KeyCode, MouseButton, OnUpdate, Plugin, Query, Res, Transform, With,
    },
    window::Window,
};

use crate::{app::AppState, collision::Collider, viewport::PrimaryCameraMarker};

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
    camera_q: Query<(&Camera, &GlobalTransform), With<PrimaryCameraMarker>>,
) {
    if windows.is_empty() {
        return;
    }
    let window = windows.single();
    if window.cursor_position().is_none() {
        return;
    }
    let cursor_pos_window = window.cursor_position().unwrap();
    let (camera, camera_transform) = camera_q.single();
    let cursor_pos = camera
        .viewport_to_world(camera_transform, cursor_pos_window)
        .unwrap()
        .origin
        .truncate();

    if input_mouse.get_just_pressed().len() > 0 {
        // get all clicked listeners
        let mut clicked: Vec<_> = listeners
            .iter_mut()
            .flat_map(|(listener, collider, xform)| {
                let dist = xform.translation.xy().distance(cursor_pos);
                if dist <= collider.radius {
                    Some((listener, xform))
                    // listener.0.send(input_mouse.clone());
                } else {
                    None
                }
            })
            .collect();
        // exit early if no components were clicked
        if clicked.is_empty() {
            return;
        }
        // send a click event to whichever is on top (Z order). If multiple
        // components have the same z value, all of them get clicked.
        let clicked_z = clicked
            .iter()
            .map(|(_, xform)| xform.translation.z)
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .unwrap();
        clicked
            .iter_mut()
            .filter(|(_, xform)| {
                xform
                    .translation
                    .z
                    .partial_cmp(&clicked_z)
                    .unwrap_or(Ordering::Equal)
                    == Ordering::Equal
            })
            .for_each(|(ref mut listener, _)| listener.0.send(input_mouse.clone()));
    }
}
