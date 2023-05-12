//! Logic related to the loading state.
//!
//! The game first loads the config and minimal loading assets in the PreLoading state.
//! Then it transitions to the Loading state, where it can load the rest of the assets.

use std::collections::HashMap;

use bevy::{
    asset::{Asset, LoadState},
    prelude::{
        info, AssetServer, Assets, Color, Commands, Component, Entity, Handle, HandleUntyped,
        Image, IntoSystemAppConfig, IntoSystemConfig, NextState, OnEnter, OnExit, OnUpdate, Plugin,
        Query, Res, ResMut, Resource, Transform, Vec3, With,
    },
    text::{Font, Text, Text2dBundle, TextStyle},
    utils::default,
};
use bevy_kira_audio::AudioSource;
use serde::Deserialize;

use crate::{app::AppState, config::Config, splash::SplashCleanup};

const CONFIG_ASSET_PATH: &str = "config.toml";

#[derive(Debug)]
pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(system_preload_setup.in_schedule(OnEnter(AppState::PreLoading)));
        app.add_system(system_preload_watch_config.in_set(OnUpdate(AppState::PreLoading)));
        app.add_system(system_loading_setup.in_schedule(OnEnter(AppState::Loading)));
        app.add_system(system_loading_update.in_set(OnUpdate(AppState::Loading)));
        app.add_system(system_loading_cleanup.in_schedule(OnExit(AppState::Loading)));
    }
}

#[derive(Debug, Resource)]
struct LoadingConfig(Handle<Config>);

#[derive(Debug, Resource)]
struct LoadingFont(Handle<Font>);

/// A map used for caching loaded assets for later use
#[derive(Debug, Resource)]
pub struct AssetMap<T: Asset>(pub HashMap<String, Handle<T>>);

/// Assets to load on startup
#[derive(Debug, Clone, Resource, Deserialize)]
pub struct AssetsConfig {
    pub images: HashMap<String, String>,
    pub audio: HashMap<String, String>,
}

#[derive(Debug, Clone, Resource, Default)]
struct AssetsLoading(Vec<HandleUntyped>);

#[derive(Debug, Clone, Component)]
struct TitleTextMarker;

#[derive(Debug, Clone, Component)]
struct StateTextMarker;

#[derive(Debug, Clone, Component)]
struct StatusTextMarker;

/// Initiate asset preloading
fn system_preload_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // load font
    let font = asset_server.load("fira_sans/FiraSans-Regular.ttf");
    // let font = asset_server.load("kenney_fonts/Kenney Future.ttf");
    commands.insert_resource(LoadingFont(font.clone()));

    // // display title text
    // let title_text_style = TextStyle {
    //     font_size: 128.0,
    //     color: Color::WHITE,
    //     font,
    // };
    // commands.spawn((
    //     Text2dBundle {
    //         text: Text::from_section("Stroids", title_text_style),
    //         ..default()
    //     },
    //     TitleTextMarker,
    // ));
    let title_font = asset_server.load("kenney_fonts/Kenney Future.ttf");
    // display title text
    let title_text_style = TextStyle {
        font_size: 256. + 128.,
        color: Color::WHITE,
        font: title_font,
    };
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Stroids", title_text_style),
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                ..default()
            },
            ..default()
        },
        SplashCleanup,
    ));

    // start loading config
    let config_handle: Handle<Config> = asset_server.load(CONFIG_ASSET_PATH);

    // insert loading state resource
    commands.insert_resource(LoadingConfig(config_handle));
}

fn system_preload_watch_config(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    loading_config: Res<LoadingConfig>,
    configs: Option<Res<Assets<Config>>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    match asset_server.get_load_state(loading_config.0.id()) {
        LoadState::Failed => {
            panic!("Loading asset failed: '{}'", CONFIG_ASSET_PATH);
        }
        LoadState::Loaded => {
            // unpack config
            let configs = configs.expect("Config loaded aut assets not present");
            let config = configs
                .get(&loading_config.0)
                .expect("Expected Config to be available after loading.");
            commands.insert_resource(config.clone());
            commands.remove_resource::<LoadingConfig>();
            next_state.set(AppState::Loading);
            info!("Config loaded");
        }
        _ => {
            // not loaded yet
        }
    }
}

fn system_loading_setup(
    mut commands: Commands,
    assets_config: Res<Config>,
    asset_server: Res<AssetServer>,
    loading_font: Res<LoadingFont>,
) {
    // set up loading tracker
    let mut loading = AssetsLoading::default();

    // load images
    let images: HashMap<String, Handle<Image>> = assets_config
        .assets
        .images
        .iter()
        .map(|(k, v)| {
            let handle: Handle<Image> = asset_server.load(v);
            // track loaded images in the loading list
            loading.0.push(handle.clone_untyped());
            (k.clone(), handle)
        })
        .collect();
    commands.insert_resource(AssetMap(images));

    // load audio
    let audio_sources: HashMap<String, Handle<AudioSource>> = assets_config
        .assets
        .audio
        .iter()
        .map(|(k, v)| {
            let handle: Handle<AudioSource> = asset_server.load(v);
            // track loaded images in the loading list
            loading.0.push(handle.clone_untyped());
            (k.clone(), handle)
        })
        .collect();
    commands.insert_resource(AssetMap(audio_sources));

    // insert loading tracker as a resource
    commands.insert_resource(loading);

    // display state text
    let state_text_style = TextStyle {
        font: loading_font.0.clone(),
        font_size: 128.0,
        color: Color::WHITE,
    };
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Loading", state_text_style),
            transform: Transform {
                translation: Vec3::new(0., -256., 0.),
                ..default()
            },
            ..default()
        },
        StateTextMarker,
    ));

    let status_text_style = TextStyle {
        font: loading_font.0.clone(),
        font_size: 64.0,
        color: Color::WHITE,
    };

    // spawn status text
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("", status_text_style),
            transform: Transform::from_translation(Vec3::new(0., -256. - 128., 0.)),
            ..default()
        },
        StatusTextMarker,
    ));
}

fn system_loading_update(
    asset_server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
    mut next_state: ResMut<NextState<AppState>>,
    mut status_text: Query<&mut Text, With<StatusTextMarker>>,
) {
    // count what's been loaded so far
    let mut loaded: usize = 0;
    for handle in loading.0.iter() {
        match asset_server.get_load_state(handle.id()) {
            LoadState::Loaded => {
                loaded += 1;
            }
            LoadState::Failed => {
                // TODO: fail more gracefully
                panic!("Failed to load asset: {:?}", handle);
            }
            _ => {} // still loading
        }
    }
    // update status text
    let status = format!("{} / {}", loaded, loading.0.len());
    status_text.single_mut().sections[0].value = status;

    if loaded == loading.0.len() {
        // TODO: go to main menu when it's implemented
        info!("All assets loaded");
        next_state.set(AppState::Splash)
    }
}

fn system_loading_cleanup(
    mut commands: Commands,
    state_text: Query<Entity, With<StateTextMarker>>,
    status_text: Query<Entity, With<StatusTextMarker>>,
) {
    state_text.iter().for_each(|e| commands.entity(e).despawn());
    status_text
        .iter()
        .for_each(|e| commands.entity(e).despawn());

    commands.remove_resource::<LoadingFont>();
    commands.remove_resource::<AssetsLoading>();
}
