use bevy::{
    prelude::{App, Camera2dBundle, ClearColor, Color, Commands, PluginGroup},
    utils::default,
    window::{Window, WindowPlugin},
    DefaultPlugins,
};

use crate::player::plugin::PlayerPlugin;

pub fn run() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.3, 0.6)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Stroids".into(),
                resolution: (800., 600.).into(),
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: false,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
