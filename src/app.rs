use bevy::{
    prelude::{
        App, Camera2dBundle, ClearColor, Color, Commands, IntoSystemAppConfig, PluginGroup, Res,
        States,
    },
    utils::default,
    DefaultPlugins,
};

use crate::{
    config::{Config, ConfigPlugin},
    input::InputPlugin,
    loading::LoadingPlugin,
    player::PlayerPlugin,
    ship::ShipPlugin,
};

/// Application State.  during development, not all of these will be implemented yet.
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    /// Load the bare essentials (such as config), so that we know what to do in the Loading state.
    #[default]
    PreLoading,
    /// Load the game's assets
    /// Unimplemented
    Loading,
    /// Show the player controls and maybe let them select their ship
    /// Unimplemented
    MainMenu,
    /// Play asteroids
    InGame,
    /// pause the game
    /// Unimplemented
    Paused,
}

pub fn run() {
    // load config
    // TODO: maybe it makes sense to load dynamically from assets? That way I'm not recompiling as often.
    let config: Config = default();
    let clear_color = config.background_color();
    let window_plugin = config.window_plugin();
    // run app
    App::new()
        .add_plugins(DefaultPlugins.set(window_plugin))
        .add_state::<AppState>()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugin(ConfigPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ShipPlugin)
        .add_plugin(LoadingPlugin)
        .add_system(spawn_camera_system.on_startup())
        .run();
}

fn spawn_camera_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
