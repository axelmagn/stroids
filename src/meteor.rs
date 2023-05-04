use std::f32::consts::PI;

use bevy::{
    prelude::{
        Bundle, Commands, Component, Image, IntoSystemAppConfig, IntoSystemConfig, OnEnter,
        OnUpdate, Plugin, Query, Res, Resource, Transform, Vec2, Vec3,
    },
    sprite::SpriteBundle,
    time::Time,
    utils::{default, HashMap},
};
use rand::{distributions::Uniform, thread_rng, Rng};
use serde::Deserialize;

use crate::{
    app::AppState,
    loading::AssetMap,
    viewport::{ViewportBounded, ViewportBounds},
};

#[derive(Debug)]
pub struct MeteorPlugin;

impl Plugin for MeteorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(MeteorBundle::system_spawn.in_schedule(OnEnter(AppState::InGame)));
        app.add_system(MeteorBundle::system_movement.in_set(OnUpdate(AppState::InGame)));
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
            MeteorSize::Large => Some(MeteorSize::Medium),
            MeteorSize::Medium => Some(MeteorSize::Small),
            MeteorSize::Small => Some(MeteorSize::Tiny),
            MeteorSize::Tiny => None,
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
}

impl Default for SizedMeteorConfig {
    fn default() -> Self {
        Self {
            sprites: Default::default(),
            speed: Default::default(),
            scale: 1.,
        }
    }
}

#[derive(Debug, Default, Component)]
struct MeteorBehavior {
    rotation_velocity: f32,
    size: MeteorSize,
    variant: String,
    velocity: Vec2,
}

#[derive(Bundle, Default)]
struct MeteorBundle {
    behavior: MeteorBehavior,
    viewport_bounded: ViewportBounded,
    #[bundle]
    sprite_bundle: SpriteBundle,
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
        let velocity = Vec2::from_angle(angle) * meteor_config.speed;

        // roll random rotation_velocity
        let rotation_velocity_max = PI / 4.;
        let rotation_velocity_dist = Uniform::new(-rotation_velocity_max, rotation_velocity_max);
        let rotation_velocity = rng.sample(rotation_velocity_dist);
        Self {
            behavior: MeteorBehavior {
                rotation_velocity,
                size,
                variant: variant_key.clone(),
                velocity,
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
            ..default()
        }
    }

    fn system_movement(mut q: Query<(&MeteorBehavior, &mut Transform)>, time: Res<Time>) {
        let dt = time.delta_seconds();
        for (behavior, mut xform) in q.iter_mut() {
            xform.translation += Vec3::from((behavior.velocity * dt, 0.));
            xform.rotate_z(behavior.rotation_velocity * dt);
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
        let bundles: Vec<MeteorBundle> = (0..10)
            .into_iter()
            .map(|_| {
                // roll position
                let x_dist = Uniform::new(viewport_bounds.0.min.x, viewport_bounds.0.max.x);
                let y_dist = Uniform::new(viewport_bounds.0.min.y, viewport_bounds.0.max.y);
                let x = rng.sample(x_dist);
                let y = rng.sample(y_dist);
                let pos = Vec3::new(x, y, 0.);

                // roll meteor
                Self::new_random(&mut rng, MeteorSize::Large, pos, &meteors_config, &images)
            })
            .collect();
        commands.spawn_batch(bundles);
    }
}
