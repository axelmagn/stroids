use bevy::prelude::{AssetServer, Commands, Res};

use super::components::PlayerBundle;

pub fn setup_player(mut commands: Commands, assets: Res<AssetServer>) {
    // TODO: configure player ship or make selectable
    commands.spawn(PlayerBundle::from(
        assets.load("space_shooter/Player/playerShip1_blue.png"),
    ));
}
