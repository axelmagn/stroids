use bevy::{
    prelude::{App, ClearColor, Color, States},
    DefaultPlugins,
};
use bevy_kira_audio::AudioPlugin;
use bevy_mod_gizmos::GizmosPlugin;

use crate::{
    collision::CollisionPlugin, config::ConfigPlugin, input::InputPlugin,
    kinematics::KinematicsPlugin, loading::LoadingPlugin, meteor::MeteorPlugin,
    player::PlayerPlugin, projectile::ProjectilePlugin, ship::ShipPlugin, sounds::SoundsPlugin,
    splash::SplashPlugin, viewport::ViewportPlugin,
};

/// Application State.  during development, not all of these will be implemented yet.
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    /// Load the bare essentials (such as config), so that we know what to do in the Loading state.
    #[default]
    PreLoading,
    /// Show a splash screen, inviting the player to click play.
    /// Clicking play triggers web initialization that requires player input.
    Splash,
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
    let clear_color = Color::hex("080811").unwrap();
    // run app
    App::new()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(GizmosPlugin)
        // .add_plugin(WorldInspectorPlugin::new())
        // Game Plugins
        .add_plugin(CollisionPlugin)
        .add_plugin(ConfigPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(KinematicsPlugin)
        .add_plugin(LoadingPlugin)
        .add_plugin(MeteorPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ProjectilePlugin)
        .add_plugin(ShipPlugin)
        .add_plugin(SoundsPlugin)
        .add_plugin(SplashPlugin)
        .add_plugin(ViewportPlugin)
        .insert_resource(ClearColor(clear_color))
        .run();
}
