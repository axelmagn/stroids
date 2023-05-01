//! Logic related to the loading state.
//!
//! The game first loads the config and minimal loading assets in the PreLoading state.
//! Then it transitions to the Loading state, where it can load the rest of the assets.

use bevy::{
    asset::LoadState,
    prelude::{
        info, AssetServer, Assets, Color, Commands, Entity, Handle, IntoSystemAppConfig,
        IntoSystemConfig, NextState, OnEnter, OnExit, OnUpdate, Plugin, Res, ResMut, Resource,
    },
    text::{Font, Text, Text2dBundle, TextStyle},
    utils::default,
};

use crate::{app::AppState, config::Config};

const CONFIG_ASSET_PATH: &'static str = "config.toml";

#[derive(Debug)]
pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(setup_preload_system.in_schedule(OnEnter(AppState::PreLoading)));
        app.add_system(watch_config_preload_system.in_set(OnUpdate(AppState::PreLoading)));
        app.add_system(cleanup_preload_system.in_schedule(OnExit(AppState::PreLoading)));
    }
}

#[derive(Debug, Resource)]
struct PreloadingScratch {
    title: Entity,
    config_handle: Handle<Config>,
}

#[derive(Debug, Resource)]
struct LoadingScratch;

/// Initiate asset preloading
fn setup_preload_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // load font
    let font = asset_server.load("fira_sans/FiraSans-Regular.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };
    // display state text
    let title = commands
        .spawn(Text2dBundle {
            text: Text::from_section("Loading", text_style.clone()),
            ..default()
        })
        .id();
    // start loading config
    let config_handle: Handle<Config> = asset_server.load(CONFIG_ASSET_PATH);

    // insert loading state resource
    commands.insert_resource(PreloadingScratch {
        title,
        config_handle,
    });
}

fn watch_config_preload_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    preload_scratch: Res<PreloadingScratch>,
    configs: Option<Res<Assets<Config>>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    match asset_server.get_load_state(preload_scratch.config_handle.id()) {
        LoadState::Failed => {
            panic!("Loading asset failed: '{}'", CONFIG_ASSET_PATH);
        }
        LoadState::Loaded => {
            // unpack config
            let configs = configs.expect("Config loaded but assets not present");
            let config = configs
                .get(&preload_scratch.config_handle)
                .expect("Expected Config to be available after loading.");
            commands.insert_resource(config.clone());
            // TEMPORARY: go straight from preloading to in-game
            next_state.set(AppState::InGame);
            // TODO: implement loading state
            // next_state.set(AppState::Loading)
        }
        _ => {
            // not loaded yet
        }
    }
}

fn cleanup_preload_system(mut commands: Commands, preload_scratch: ResMut<PreloadingScratch>) {
    commands.entity(preload_scratch.title).despawn();
    commands.remove_resource::<PreloadingScratch>();
}
