use bevy::prelude::{App, Plugin};

use super::systems::setup_player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_player);
    }
}
