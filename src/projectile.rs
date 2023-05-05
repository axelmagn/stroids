use bevy::{
    math::Vec3Swizzles,
    prelude::{
        Bundle, Commands, Component, Entity, Image, IntoSystemConfig, OnUpdate, Plugin, Query, Res,
        Resource, Transform, Vec3,
    },
    sprite::SpriteBundle,
    time::{Time, Timer, TimerMode},
    utils::default,
};
use serde::Deserialize;

use crate::{
    app::AppState,
    collision::Collider,
    kinematics::{KinematicsBundle, Velocity},
    loading::AssetMap,
    viewport::ViewportBounded,
};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(Self::system_lifetime.in_set(OnUpdate(AppState::InGame)));
    }
}

impl ProjectilePlugin {
    fn system_lifetime(
        mut commands: Commands,
        mut q: Query<(Entity, &mut ProjectileComponent)>,
        time: Res<Time>,
    ) {
        // info!("projectile lifetime (t={})", time.elapsed_seconds());
        q.iter_mut().for_each(|(e, mut pc)| {
            pc.lifetime.tick(time.delta());
            // DEBUG
            // info!("{:?}: {:?}", e, pc.lifetime.elapsed());
            if pc.lifetime.finished() {
                commands.entity(e).despawn();
            }
        });
    }
}

#[derive(Debug, Clone, Resource, Deserialize)]
pub struct ProjectileConfig {
    sprite_id: String,
    speed: f32,
    collision_radius: f32,
    scale: f32,
    lifetime: f32,
}

#[derive(Debug, Clone, Component)]
pub struct ProjectileComponent {
    lifetime: Timer,
}

#[derive(Clone, Bundle)]
pub struct ProjectileBundle {
    projectile: ProjectileComponent,
    collider: Collider,
    bounded: ViewportBounded,
    #[bundle]
    sprite: SpriteBundle,
    #[bundle]
    kinematics: KinematicsBundle,
}

impl ProjectileBundle {
    pub fn from_config(
        config: &ProjectileConfig,
        xform: &Transform,
        sprites: &AssetMap<Image>,
    ) -> Self {
        let texture = sprites
            .0
            .get(&config.sprite_id)
            .unwrap_or_else(|| panic!("could not find sprite_id: ({})", config.sprite_id))
            .clone();
        let direction = xform.up().xy();
        let velocity = Velocity(direction * config.speed);
        Self {
            projectile: ProjectileComponent {
                lifetime: Timer::from_seconds(config.lifetime, TimerMode::Once),
            },
            collider: Collider {
                radius: config.collision_radius,
            },
            bounded: ViewportBounded,
            sprite: SpriteBundle {
                texture,
                transform: Transform {
                    translation: xform.translation,
                    rotation: xform.rotation,
                    scale: Vec3::ONE * config.scale,
                },
                ..default()
            },
            kinematics: KinematicsBundle {
                velocity,
                ..default()
            },
        }
    }
}
