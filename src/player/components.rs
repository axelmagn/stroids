use bevy::{
    prelude::{Bundle, Component, Handle, Image},
    sprite::SpriteBundle,
    utils::default,
};

#[derive(Component, Default)]
pub struct PlayerMarker;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    _p: PlayerMarker,

    #[bundle]
    sprite: SpriteBundle,
}

impl PlayerBundle {
    pub fn from(sprite: Handle<Image>) -> Self {
        PlayerBundle {
            sprite: SpriteBundle {
                texture: sprite,
                ..default()
            },
            ..default()
        }
    }
}
