use bevy::{
    input::ButtonState,
    prelude::{
        App, AssetServer, Bundle, Commands, Component, EventReader, Plugin, Query, Res, With,
    },
};

use crate::{
    config::Config,
    input::{InputAction, InputEvent},
    ship::{ShipBundle, ShipControls},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player_system);
        app.add_system(handle_player_input_system);
    }
}

#[derive(Component, Default, Debug)]
pub struct PlayerMarker;

#[derive(Component, Default, Debug)]
pub struct PlayerInputMemory {
    pub thrust: Option<InputEvent>,
    pub turn: Option<InputEvent>,
}

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    _p: PlayerMarker,
    input_memory: PlayerInputMemory,

    #[bundle]
    ship: ShipBundle,
}

pub fn spawn_player_system(mut commands: Commands, assets: Res<AssetServer>, config: Res<Config>) {
    let sprite_path = &config.assets.player_ship;
    let mut player = PlayerBundle::default();
    player.ship.sprite.texture = assets.load(sprite_path);
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