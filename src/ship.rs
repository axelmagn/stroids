use bevy::{
    math::Vec3Swizzles,
    prelude::{
        Bundle, Component, IntoSystemConfig, OnUpdate, Plugin, Query, Transform, Vec2, Vec3,
    },
    reflect::Reflect,
    sprite::SpriteBundle,
};
use serde::Deserialize;

use crate::{
    app::AppState,
    collision::Collider,
    kinematics::{Acceleration, AngularAcceleration, KinematicsBundle},
};

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<ShipControls>();
        app.register_type::<ShipConfig>();
        app.add_system(Self::system_ship_controls.in_set(OnUpdate(AppState::InGame)));
    }
}

impl ShipPlugin {
    fn system_ship_controls(
        mut q: Query<(
            &ShipControls,
            &ShipConfig,
            &Transform,
            &mut Acceleration,
            &mut AngularAcceleration,
        )>,
    ) {
        q.iter_mut()
            .for_each(|(controls, config, xform, mut acc, mut racc)| {
                acc.0 = xform.up().xy() * controls.thrust * config.thrust_factor;
                racc.0 = controls.turn * config.turn_factor;
            });
    }
}

#[derive(Reflect, Component, Default, Debug)]
pub struct ShipControls {
    /// thrust input [-1,1]
    pub thrust: f32,
    /// turn input [-1, 1]
    pub turn: f32,
}

#[derive(Reflect, Component, Clone, Debug, Default, Deserialize)]
pub struct ShipConfig {
    pub thrust_factor: f32,
    pub turn_factor: f32,
    pub velocity_damping: f32,
    pub rotation_rate_damping: f32,
    pub sprite_id: String,
    pub collision_radius: f32,
}

#[derive(Bundle, Default)]
pub struct ShipBundle {
    pub controls: ShipControls,
    pub config: ShipConfig,
    pub collider: Collider,

    #[bundle]
    pub sprite: SpriteBundle,

    #[bundle]
    pub kinematics: KinematicsBundle,
}
