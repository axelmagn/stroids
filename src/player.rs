use bevy::{
    input::ButtonState,
    prelude::{
        App, Bundle, Commands, Component, EventReader, Image, IntoSystemAppConfig,
        IntoSystemConfig, OnEnter, OnUpdate, Plugin, Query, Res, Resource, With,
    },
    utils::default,
};
use serde::Deserialize;

use crate::{
    app::AppState,
    input::{InputAction, InputEvent},
    loading::AssetMap,
    ship::{ShipBundle, ShipConfig, ShipControls},
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

    #[bundle]
    ship: ShipBundle,
}

pub fn spawn_player_system(
    mut commands: Commands,
    loaded_images: Res<AssetMap<Image>>,
    config: Res<PlayerConfig>,
) {
    // TODO: get ShipBundle from ShipConfig
    let mut player = PlayerBundle {
        ship: ShipBundle {
            config: config.ship.clone(),
            ..default()
        },
        ..default()
    };
    let sprite_id = &config.ship.sprite_id;
    let err_msg = format!("Could not find player sprite: {}", sprite_id);
    player.ship.sprite.texture = loaded_images.0.get(sprite_id).expect(&err_msg).clone();
    commands.spawn(player);
}

pub fn handle_player_input_system(
    mut q: Query<(&mut ShipControls, &mut PlayerInputMemory), (With<PlayerMarker>,)>,
    mut evr_inputs: EventReader<InputEvent>,
) {
    let (mut controls, mut input_mem) = q.single_mut();
    evr_inputs
        .iter()
        .for_each(|ev_input| match ev_input.action {
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
