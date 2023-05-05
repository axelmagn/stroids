use bevy::{
    math::Vec3Swizzles,
    prelude::{
        info, Bundle, Commands, Component, Image, IntoSystemConfig, OnUpdate, Plugin, Query, Res,
        Transform,
    },
    reflect::Reflect,
    sprite::SpriteBundle,
    time::{Time, Timer},
};
use serde::Deserialize;

use crate::{
    app::AppState,
    collision::Collider,
    kinematics::{Acceleration, AngularAcceleration, KinematicsBundle},
    loading::AssetMap,
    projectile::{ProjectileBundle, ProjectileConfig},
};

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<ShipControls>();
        app.register_type::<ShipConfig>();
        app.add_system(Self::system_handle_controls.in_set(OnUpdate(AppState::InGame)));
        app.add_system(Self::system_shoot_cooldown.in_set(OnUpdate(AppState::InGame)));
    }
}

impl ShipPlugin {
    fn system_handle_controls(
        mut commands: Commands,
        mut q: Query<(
            &ShipControls,
            &mut ShootCooldown,
            &ShipConfig,
            &Transform,
            &mut Acceleration,
            &mut AngularAcceleration,
        )>,
        projectile_config: Res<ProjectileConfig>,
        sprites: Res<AssetMap<Image>>,
    ) {
        q.iter_mut().for_each(
            |(controls, mut shoot_cooldown, config, xform, mut acc, mut racc)| {
                // update kinematics
                let direction = xform.up().xy();
                acc.0 = direction * controls.thrust * config.thrust_factor;
                racc.0 = controls.turn * config.turn_factor;
                // handle shooting
                if controls.shoot && shoot_cooldown.0.finished() {
                    let projectile =
                        ProjectileBundle::from_config(&projectile_config, &xform, &sprites);
                    commands.spawn(projectile);
                    shoot_cooldown.0.reset();
                    // TODO: projectile kickback
                }
            },
        );
    }

    fn system_shoot_cooldown(mut q: Query<&mut ShootCooldown>, time: Res<Time>) {
        q.iter_mut().for_each(|mut t| {
            t.0.tick(time.delta());
        });
    }
}

#[derive(Reflect, Component, Default, Debug)]
pub struct ShipControls {
    /// thrust input [-1,1]
    pub thrust: f32,
    /// turn input [-1, 1]
    pub turn: f32,
    pub shoot: bool,
}

#[derive(Debug, Component)]
pub struct ShootCooldown(pub Timer);

#[derive(Reflect, Component, Clone, Debug, Default, Deserialize)]
pub struct ShipConfig {
    pub thrust_factor: f32,
    pub turn_factor: f32,
    pub velocity_damping: f32,
    pub rotation_rate_damping: f32,
    pub sprite_id: String,
    pub collision_radius: f32,
    pub shoot_cooldown: f32,
}

#[derive(Bundle)]
pub struct ShipBundle {
    pub controls: ShipControls,
    pub config: ShipConfig,
    pub collider: Collider,
    pub shoot_cooldown: ShootCooldown,

    #[bundle]
    pub sprite: SpriteBundle,

    #[bundle]
    pub kinematics: KinematicsBundle,
}
