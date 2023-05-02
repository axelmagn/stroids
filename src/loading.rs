//! Logic related to the loading state.
//!
//! The game first loads the config and minimal loading assets in the PreLoading state.
//! Then it transitions to the Loading state, where it can load the rest of the assets.

use std::{collections::HashMap, path::Component};

use bevy::{
    asset::{Asset, LoadState},
    prelude::{
        info, AssetServer, Assets, Color, Commands, Component, Entity, Handle, HandleUntyped,
        Image, IntoSystemAppConfig, IntoSystemConfig, NextState, OnEnter, OnExit, OnUpdate, Plugin,
        Res, ResMut, Resource,
    },
    text::{Text, Text2dBundle, TextStyle},
    utils::default,
};
use serde::Deserialize;

use crate::{app::AppState, config::Config};

const CONFIG_ASSET_PATH: &'static str = "config.toml";

#[derive(Debug)]
pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(system_preload_setup.in_schedule(OnEnter(AppState::PreLoading)));
        app.add_system(system_preload_watch_config.in_set(OnUpdate(AppState::PreLoading)));
        app.add_system(system_preload_cleanup.in_schedule(OnExit(AppState::PreLoading)));
    }
}

#[derive(Debug, Resource)]
struct PreloadingScratch {
    title: Entity,
    config_handle: Handle<Config>,
}

#[derive(Debug, Resource)]
struct LoadingScratch;

/// A map used for caching loaded assets for later use
#[derive(Debug, Resource)]
struct AssetMap<T: Asset>(HashMap<String, Handle<T>>);

/// Assets to load on startup
#[derive(Debug, Clone, Resource, Deserialize)]
pub struct AssetsConfig {
    pub images: HashMap<String, String>,
}

#[derive(Debug, Clone, Resource, Default)]
struct AssetsLoading(Vec<HandleUntyped>);

#[derive(Debug, Clone, Component)]
struct TitleTextMarker;

#[derive(Debug, Clone, Component)]
struct StatusTextMarker;

/// Initiate asset preloading
fn system_preload_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // load font
    let font = asset_server.load("fira_sans/FiraSans-Regular.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };
    // display state text
    let title = commands
        .spawn((
            Text2dBundle {
                text: Text::from_section("Loading", text_style.clone()),
                ..default()
            },
            TitleTextMarker,
        ))
        .id();
    // start loading config
    let config_handle: Handle<Config> = asset_server.load(CONFIG_ASSET_PATH);

    // insert loading state resource
    commands.insert_resource(PreloadingScratch {
        title,
        config_handle,
    });
}

fn system_preload_watch_config(
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
            let configs = configs.expect("Config loaded aut assets not present");
            let config = configs
                .get(&preload_scratch.config_handle)
                .expect("Expected Config to be available after loading.");
            commands.insert_resource(config.clone());
            // TEMPORARY: go straight from preloading to in-game
            next_state.set(AppState::InGame);
            info!("Config Loaded");
            // TODO: implement loading state
            // next_state.set(AppState::Loading)
        }
        _ => {
            // not loaded yet
        }
    }
}

fn system_preload_cleanup(mut commands: Commands, preload_scratch: ResMut<PreloadingScratch>) {
    commands.entity(preload_scratch.title).despawn();
    commands.remove_resource::<PreloadingScratch>();
}

fn system_loading_setup(
    mut commands: Commands,
    assets_config: Res<AssetsConfig>,
    asset_server: Res<AssetServer>,
) {
    // set up status tracker

    // set up loading tracker
    let mut loading = AssetsLoading::default();

    // load images
    let images: HashMap<String, Handle<Image>> = assets_config
        .images
        .iter()
        .map(|(k, v)| {
            let handle: Handle<Image> = asset_server.load(v);
            loading.0.push(handle.clone_untyped());
            (k.clone(), handle)
        })
        .collect();
    commands.insert_resource(AssetMap(images));

    // insert loading tracker as a resource
    commands.insert_resource(loading);
}

fn system_loading_update(asset_server: Res<AssetServer>, loading: Res<AssetsLoading>) {
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
}

fn system_loading_cleanup() {}
