use bevy::{
    math::Vec3Swizzles,
    prelude::{
        Bundle, Component, IntoSystemConfig, OnUpdate, Plugin, Query, Res, Transform, Vec2, Vec3,
    },
    reflect::Reflect,
    sprite::SpriteBundle,
    time::Time,
};
use serde::Deserialize;

use crate::app::AppState;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<ShipControls>();
        app.register_type::<ShipConfig>();
        app.register_type::<ShipState>();
        app.add_system(apply_ship_controls_system.in_set(OnUpdate(AppState::InGame)));
        app.add_system(update_ship_physics_system.in_set(OnUpdate(AppState::InGame)));
    }
}

#[derive(Reflect, Component, Default, Debug)]
pub struct ShipControls {
    /// thrust input [-1,1]
    pub thrust: f32,
    /// turn input [-1, 1]
    pub turn: f32,
}

#[derive(Reflect, Component, Clone, Debug, Deserialize)]
pub struct ShipConfig {
    pub thrust_factor: f32,
    pub turn_factor: f32,
    pub velocity_damping: f32,
    pub rotation_rate_damping: f32,
    pub sprite_id: String,
}

// TODO: delete
impl Default for ShipConfig {
    fn default() -> Self {
        Self {
            thrust_factor: 400.0,
            turn_factor: 7.0,
            velocity_damping: 0.3,
            rotation_rate_damping: 0.7,
            sprite_id: "".to_string(),
        }
    }
}

#[derive(Reflect, Component, Default, Debug)]
pub struct ShipState {
    /// angle in radians
    pub velocity: Vec2,
    pub rotation_rate: f32,
}

#[derive(Bundle, Default)]
pub struct ShipBundle {
    pub controls: ShipControls,
    pub config: ShipConfig,
    pub state: ShipState,

    #[bundle]
    pub sprite: SpriteBundle,
}

/// Apply ship controls to ship state
pub fn apply_ship_controls_system(
    time: Res<Time>,
    mut q: Query<(&ShipControls, &ShipConfig, &mut ShipState, &Transform)>,
) {
    for (controls, config, mut state, transform) in q.iter_mut() {
        let dt = time.delta_seconds();

        // apply rotation
        state.rotation_rate += controls.turn * config.turn_factor * dt;

        // apply thrust
        let accel_dir = transform.up().xy();
        let accel_mag = controls.thrust * config.thrust_factor * dt;
        let accel = accel_dir * accel_mag;
        state.velocity += accel;
    }
}

pub fn update_ship_physics_system(
    time: Res<Time>,
    mut q: Query<(&ShipConfig, &mut ShipState, &mut Transform)>,
) {
    for (config, mut state, mut transform) in q.iter_mut() {
        let dt = time.delta_seconds();

        // apply rotation damping
        state.rotation_rate *= 1. - config.rotation_rate_damping * dt;

        // apply velocity damping
        state.velocity *= 1. - config.velocity_damping * dt;

        // apply velocity to position
        transform.translation += Vec3::from((state.velocity * dt, 0.));

        // apply rotation rate to rotation
        transform.rotate_z(state.rotation_rate * dt);
    }
}
