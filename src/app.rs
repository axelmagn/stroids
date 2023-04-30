use bevy::{
    prelude::{App, ClearColor, Commands, PluginGroup, Res},
    utils::default,
    DefaultPlugins,
};

use crate::{config::Config, input::InputPlugin, player::PlayerPlugin, ship::ShipPlugin};

pub fn run() {
    // load config
    // TODO: maybe it makes sense to load dynamically from assets? That way I'm not recompiling as often.
    let config: Config = default();
    let clear_color = config.background_color();
    let window_plugin = config.window_plugin();
    // run app
    App::new()
        .insert_resource(config)
        .insert_resource(ClearColor(clear_color))
        .add_plugins(DefaultPlugins.set(window_plugin))
        .add_plugin(InputPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ShipPlugin)
        .add_startup_system(spawn_camera_system)
        .run();
}

fn spawn_camera_system(mut commands: Commands, config: Res<Config>) {
    commands.spawn(config.camera());
}
