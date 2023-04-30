use std::str::from_utf8;

use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::{Camera2dBundle, Color, OrthographicProjection, Resource},
    reflect::TypeUuid,
    utils::{default, BoxedFuture},
    window::{Window, WindowPlugin},
};
use serde::Deserialize;

const DEFAULT_CONFIG_STR: &'static str = include_str!("include_data/default_config.toml");

#[derive(Deserialize, Debug, TypeUuid, Resource)]
#[uuid = "4f602f8f-3160-4369-a4c4-062a031ad23b"]
pub struct Config {
    pub assets: AssetsConfig,
    pub window: WindowConfig,
    pub camera: CameraConfig,
}

#[derive(Deserialize, Debug)]
pub struct AssetsConfig {
    pub player_ship: String,
}

#[derive(Deserialize, Debug)]
pub struct WindowConfig {
    title: String,
    fit_canvas_to_parent: bool,
    prevent_default_event_handling: bool,
    resolution: [f32; 2],
    background_color: String,
}

#[derive(Deserialize, Debug)]
pub struct CameraConfig {
    pub scale: f32,
}

impl Config {
    pub fn window_plugin(&self) -> WindowPlugin {
        WindowPlugin {
            primary_window: Some(Window {
                title: self.window.title.clone(),
                resolution: self.window.resolution.into(),
                fit_canvas_to_parent: self.window.fit_canvas_to_parent,
                prevent_default_event_handling: self.window.prevent_default_event_handling,
                ..default()
            }),
            ..default()
        }
    }

    pub fn background_color(&self) -> Color {
        Color::hex(&self.window.background_color).unwrap()
    }

    pub fn camera(&self) -> Camera2dBundle {
        Camera2dBundle {
            projection: OrthographicProjection {
                scale: self.camera.scale,
                ..default()
            },
            ..default()
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        toml::from_str(DEFAULT_CONFIG_STR).expect("Could not parse default config")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_window_plugin_succeeds() {
        let window_plugin = Config::default().window_plugin();
        assert!(window_plugin.primary_window.is_some());
    }

    #[test]
    fn test_config_background_color_succeeds() {
        let _color = Config::default().background_color();
    }

    #[test]
    fn test_config_default_succeeds() {
        let config = Config::default();
        assert!(!config.assets.player_ship.is_empty());
    }
}

#[derive(Default)]
pub struct ConfigLoader;

impl AssetLoader for ConfigLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            // TODO: handle error
            let config_str = from_utf8(bytes)?;
            let config = toml::from_str(&config_str)?;
            load_context.set_default_asset(LoadedAsset::new(config));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["toml"]
    }
}
