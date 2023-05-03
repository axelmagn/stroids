use bevy::utils::HashMap;
use serde::Deserialize;

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize)]
pub enum MeteorSize {
    Tiny,
    Small,
    Medium,
    #[default]
    Large,
}

/// Configuration for a meteor.
/// Different meteor varieties (brown / grey) have different configs.
#[derive(Debug, Default, Clone, Deserialize)]
pub struct MeteorConfig(HashMap<MeteorSize, SizedMeteorConfig>);

/// Configuration for a meteor of a particular size.
#[derive(Debug, Default, Clone, Deserialize)]
pub struct SizedMeteorConfig {
    sprites: Vec<String>,
    speed: f32,
}
