use bevy::{
    prelude::{Component, Resource},
    utils::HashMap,
};
use serde::Deserialize;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, Deserialize)]
enum MeteorSize {
    Tiny,
    Small,
    Medium,
    #[default]
    Large,
}

impl MeteorSize {
    fn get_smaller(&self) -> Option<MeteorSize> {
        match self {
            MeteorSize::Large => Some(MeteorSize::Medium),
            MeteorSize::Medium => Some(MeteorSize::Small),
            MeteorSize::Small => Some(MeteorSize::Tiny),
            MeteorSize::Tiny => None,
        }
    }
}

#[derive(Debug, Default, Clone, Deserialize, Resource)]
pub struct MeteorsConfig(HashMap<String, MeteorConfig>);

/// Configuration for a meteor.
/// Different meteor varieties (brown / grey) have different configs.
#[derive(Debug, Default, Clone, Deserialize)]
struct MeteorConfig(HashMap<MeteorSize, SizedMeteorConfig>);

/// Configuration for a meteor of a particular size.
#[derive(Debug, Default, Clone, Deserialize)]
struct SizedMeteorConfig {
    sprites: Vec<String>,
    speed: f32,
}

#[derive(Debug, Default, Component)]
struct MeteorMarker;

#[derive(Bundle, Default)]
struct MeteorBundle {
    _marker: MeteorMarker,
    #[bundle]
    sprite: SpriteBundle,
}
