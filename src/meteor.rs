use std::f32::consts::PI;

use bevy::{
    prelude::{
        Bundle, Commands, Component, Entity, Image, IntoSystemAppConfig, IntoSystemConfig, OnEnter,
        OnUpdate, Plugin, Query, Res, Resource, Transform, Vec2, Vec3, With,
    },
    sprite::SpriteBundle,
    time::Time,
    utils::{default, HashMap},
};
use rand::{distributions::Uniform, thread_rng, Rng};
use serde::Deserialize;

use crate::{
    app::AppState,
    collision::Collider,
    kinematics::{AngularVelocity, KinematicsBundle, Velocity},
    loading::AssetMap,
    player::PlayerMarker,
    viewport::{ViewportBounded, ViewportBounds},
};

#[derive(Debug)]
pub struct MeteorPlugin;

impl Plugin for MeteorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(MeteorBundle::system_spawn.in_schedule(OnEnter(AppState::InGame)));
        app.add_system(
            MeteorBundle::system_handle_player_collision.in_set(OnUpdate(AppState::InGame)),
        );
    }
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, Deserialize)]
enum MeteorSize {
    Tiny,
    Small,
    Medium,
    #[default]
    Large,
}

impl MeteorSize {
    fn smaller(&self) -> Option<MeteorSize> {
        match self {
            Self::Large => Some(Self::Medium),
            Self::Medium => Some(Self::Small),
            Self::Small => Some(Self::Tiny),
            Self::Tiny => None,
        }
    }

    fn can_split(&self) -> bool {
        match self {
            Self::Medium | Self::Large => true,
            _ => false,
        }
    }
}

#[derive(Debug, Default, Clone, Deserialize, Resource)]
pub struct MeteorsConfig {
    variants: HashMap<String, MeteorConfig>,
}

/// Configuration for a meteor.
/// Different meteor varieties (brown / grey) have different configs.
#[derive(Debug, Default, Clone, Deserialize)]
struct MeteorConfig(HashMap<MeteorSize, SizedMeteorConfig>);

/// Configuration for a meteor of a particular size.
#[derive(Debug, Clone, Deserialize)]
struct SizedMeteorConfig {
    sprites: Vec<String>,
    speed: f32,
    scale: f32,
    collision_radius: f32,
}

impl Default for SizedMeteorConfig {
    fn default() -> Self {
        Self {
            sprites: Default::default(),
            speed: Default::default(),
            scale: 1.,
            collision_radius: 100.,
        }
    }
}

#[derive(Debug, Default, Component)]
struct MeteorBehavior {
    size: MeteorSize,
    variant: String,
}

#[derive(Bundle, Default)]
struct MeteorBundle {
    behavior: MeteorBehavior,
    viewport_bounded: ViewportBounded,
    collider: Collider,
    #[bundle]
    sprite_bundle: SpriteBundle,
    #[bundle]
    kinematics: KinematicsBundle,
}

impl MeteorBundle {
    fn new_random<R: Rng>(
        rng: &mut R,
        size: MeteorSize,
        location: Vec3,
        config: &MeteorsConfig,
        images: &AssetMap<Image>,
    ) -> Self {
        // roll random meteor variant
        let variant_dist = Uniform::new(0, config.variants.len());
        let variant_idx = rng.sample(variant_dist);
        let variant_key = config.variants.keys().take(variant_idx + 1).last().unwrap();
        let meteor_config = &config.variants[variant_key].0[&size];

        // roll random sprite from variant
        let sprite_dist = Uniform::new(0, meteor_config.sprites.len());
        let sprite_idx = rng.sample(sprite_dist);
        let sprite_id = &meteor_config.sprites[sprite_idx];
        let sprite_handle = images.0[sprite_id].clone();

        // roll random direction
        let angle_dist = Uniform::new(0., PI * 2.);
        let angle = rng.sample(angle_dist);
        let velocity = Velocity(Vec2::from_angle(angle) * meteor_config.speed);

        // roll random rotation_velocity
        let angular_velocity_max = PI / 4.;
        let angular_velocity_dist = Uniform::new(-angular_velocity_max, angular_velocity_max);
        let angular_velocity = AngularVelocity(rng.sample(angular_velocity_dist));

        // create bundle
        Self {
            behavior: MeteorBehavior {
                size,
                variant: variant_key.clone(),
            },
            sprite_bundle: SpriteBundle {
                texture: sprite_handle,
                transform: Transform {
                    translation: location,
                    scale: Vec3::ONE * meteor_config.scale,
                    ..default()
                },
                ..default()
            },
            collider: Collider {
                radius: meteor_config.collision_radius * meteor_config.scale,
            },
            kinematics: KinematicsBundle {
                velocity,
                angular_velocity,
                ..default()
            },
            ..default()
        }
    }

    fn system_spawn(
        mut commands: Commands,
        meteors_config: Res<MeteorsConfig>,
        images: Res<AssetMap<Image>>,
        viewport_bounds: Res<ViewportBounds>,
    ) {
        let mut rng = thread_rng();

        // TODO: level progression
        let bundles: Vec<MeteorBundle> = (0..4)
            .into_iter()
            .map(|_| {
                // roll position - make sure it's not too close to the center where the player is
                // TODO: make configurable
                let mut pos = Vec3::ZERO;
                while pos.distance(Vec3::ZERO) < 300. {
                    let x_dist = Uniform::new(viewport_bounds.0.min.x, viewport_bounds.0.max.x);
                    let y_dist = Uniform::new(viewport_bounds.0.min.y, viewport_bounds.0.max.y);
                    let x = rng.sample(x_dist);
                    let y = rng.sample(y_dist);
                    pos = Vec3::new(x, y, 0.);
                }

                // roll meteor
                Self::new_random(&mut rng, MeteorSize::Large, pos, &meteors_config, &images)
            })
            .collect();
        commands.spawn_batch(bundles);
    }

    fn system_handle_player_collision(
        mut commands: Commands,
        q_meteors: Query<(&Transform, &Collider), With<MeteorBehavior>>,
        q_player: Query<(Entity, &Transform, &Collider), With<PlayerMarker>>,
    ) {
        for (player_entity, player_xform, player_collider) in q_player.iter() {
            for (meteor_xform, meteor_collider) in q_meteors.iter() {
                if Collider::is_collision(
                    (player_xform, player_collider),
                    (meteor_xform, meteor_collider),
                ) {
                    // TODO: emit & handle player death event
                    commands.entity(player_entity).despawn();
                }
            }
        }
    }
}
