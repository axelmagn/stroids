use bevy::{
    input::ButtonState,
    prelude::{
        App, Bundle, Commands, Component, EventReader, Image, IntoSystemAppConfig,
        IntoSystemConfig, OnEnter, OnUpdate, Plugin, Query, Res, Resource, With,
    },
    sprite::SpriteBundle,
    utils::default,
};
use serde::Deserialize;

use crate::{
    app::AppState,
    collision::Collider,
    input::{InputAction, InputEvent},
    kinematics::{AngularDamping, KinematicsBundle, LinearDamping},
    loading::AssetMap,
    ship::{ShipBundle, ShipConfig, ShipControls},
    viewport::ViewportBounded,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player_system.in_schedule(OnEnter(AppState::InGame)));
        app.add_system(handle_player_input_system.in_set(OnUpdate(AppState::InGame)));
    }
}

#[derive(Component, Default, Debug)]
pub struct PlayerMarker;

#[derive(Component, Default, Debug)]
pub struct PlayerInputMemory {
    pub thrust: Option<InputEvent>,
    pub turn: Option<InputEvent>,
}

#[derive(Clone, Debug, Deserialize, Resource)]
pub struct PlayerConfig {
    pub ship: ShipConfig,
}

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    marker: PlayerMarker,
    input_memory: PlayerInputMemory,
    viewport_bounded: ViewportBounded,

    #[bundle]
    ship: ShipBundle,
}

pub fn spawn_player_system(
    mut commands: Commands,
    loaded_images: Res<AssetMap<Image>>,
    config: Res<PlayerConfig>,
) {
    // TODO: get ShipBundle from ShipConfig
    let sprite_id = &config.ship.sprite_id;
    let err_msg = format!("Could not find player sprite: {}", sprite_id);
    let sprite_tex = loaded_images.0.get(sprite_id).expect(&err_msg).clone();
    let player = PlayerBundle {
        ship: ShipBundle {
            config: config.ship.clone(),
            collider: Collider {
                radius: config.ship.collision_radius,
            },
            kinematics: KinematicsBundle {
                linear_damping: LinearDamping(config.ship.velocity_damping),
                angular_damping: AngularDamping(config.ship.rotation_rate_damping),
                ..default()
            },
            sprite: SpriteBundle {
                texture: sprite_tex,
                ..default()
            },
            ..default()
        },
        ..default()
    };
    commands.spawn(player);
}

pub fn handle_player_input_system(
    mut q: Query<(&mut ShipControls, &mut PlayerInputMemory), (With<PlayerMarker>,)>,
    mut evr_inputs: EventReader<InputEvent>,
) {
    let inputs: Vec<InputEvent> = evr_inputs.iter().map(|e| e.clone()).collect();
    for (mut controls, mut input_mem) in q.iter_mut() {
        inputs.iter().for_each(|ev_input| match ev_input.action {
            InputAction::Thrust(x) => {
                match (ev_input.state, input_mem.thrust) {
                    (ButtonState::Pressed, _) => {
                        controls.thrust = x.clamp(-1., 1.);
                        input_mem.thrust = Some(ev_input.clone());
                    }
                    (ButtonState::Released, Some(prev)) => {
                        if ev_input.scan_code == prev.scan_code {
                            controls.thrust = 0.;
                        }
                    }
                    _ => {}
                };
                // DEBUG
                // info!("player thrust input handled: {:?}", ev_input);
                // info!("new controls state: {:?}", controls);
            }
            InputAction::Turn(x) => match (ev_input.state, input_mem.turn) {
                (ButtonState::Pressed, _) => {
                    controls.turn = x.clamp(-1., 1.);
                    input_mem.turn = Some(ev_input.clone());
                }
                (ButtonState::Released, Some(prev)) => {
                    if ev_input.scan_code == prev.scan_code {
                        controls.turn = 0.
                    }
                }
                _ => {}
            },
        });
    }
}
