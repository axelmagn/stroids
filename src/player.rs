use bevy::{
    input::ButtonState,
    prelude::{
        App, Bundle, Commands, Component, EventReader, Image, IntoSystemAppConfig,
        IntoSystemConfig, OnEnter, OnUpdate, Plugin, Query, Res, Resource, With,
    },
    sprite::SpriteBundle,
    time::{Timer, TimerMode},
    utils::default,
};
use serde::Deserialize;

use crate::{
    app::AppState,
    collision::Collider,
    input::{InputAction, InputEvent},
    kinematics::{AngularDamping, KinematicsBundle, LinearDamping},
    loading::AssetMap,
    ship::{ShipBundle, ShipConfig, ShipControls, ShootCooldown},
    viewport::ViewportBounded,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(system_spawn.in_schedule(OnEnter(AppState::InGame)));
        app.add_system(system_handle_input.in_set(OnUpdate(AppState::InGame)));
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

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: PlayerMarker,
    input_memory: PlayerInputMemory,
    viewport_bounded: ViewportBounded,

    #[bundle]
    ship: ShipBundle,
}

pub fn system_spawn(
    mut commands: Commands,
    loaded_images: Res<AssetMap<Image>>,
    config: Res<PlayerConfig>,
) {
    // TODO: get ShipBundle from ShipConfig
    let sprite_id = &config.ship.sprite_id;
    let err_msg = format!("Could not find player sprite: {}", sprite_id);
    let sprite_tex = loaded_images.0.get(sprite_id).expect(&err_msg).clone();
    let player = PlayerBundle {
        marker: PlayerMarker,
        input_memory: PlayerInputMemory::default(),
        viewport_bounded: ViewportBounded,
        ship: ShipBundle {
            controls: ShipControls::default(),
            config: config.ship.clone(),
            collider: Collider {
                radius: config.ship.collision_radius,
            },
            shoot_cooldown: ShootCooldown(Timer::from_seconds(
                config.ship.shoot_cooldown,
                TimerMode::Once,
            )),
            sprite: SpriteBundle {
                texture: sprite_tex,
                ..default()
            },
            kinematics: KinematicsBundle {
                linear_damping: LinearDamping(config.ship.velocity_damping),
                angular_damping: AngularDamping(config.ship.rotation_rate_damping),
                ..default()
            },
        },
    };
    commands.spawn(player);
}

pub fn system_handle_input(
    mut q: Query<(&mut ShipControls, &mut PlayerInputMemory), (With<PlayerMarker>,)>,
    mut evr_inputs: EventReader<InputEvent>,
) {
    let inputs: Vec<InputEvent> = evr_inputs.iter().copied().collect();
    for (mut controls, mut input_mem) in q.iter_mut() {
        inputs.iter().for_each(|ev_input| match ev_input.action {
            InputAction::Thrust(x) => {
                match (ev_input.state, input_mem.thrust) {
                    (ButtonState::Pressed, _) => {
                        controls.thrust = x.clamp(-1., 1.);
                        input_mem.thrust = Some(*ev_input);
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
                    input_mem.turn = Some(*ev_input);
                }
                (ButtonState::Released, Some(prev)) => {
                    if ev_input.scan_code == prev.scan_code {
                        controls.turn = 0.
                    }
                }
                _ => {}
            },
            InputAction::Shoot => match ev_input.state {
                ButtonState::Pressed => controls.shoot = true,
                ButtonState::Released => controls.shoot = false,
            },
        });
    }
}
