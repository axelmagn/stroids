use std::str::from_utf8;

use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::{info, AddAsset, Commands, DetectChanges, Plugin, Res, Resource},
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use serde::Deserialize;

use crate::{loading::AssetsConfig, player::PlayerConfig, viewport::ViewportConfig};

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_asset::<Config>();
        app.init_asset_loader::<ConfigLoader>();
        app.add_system(Config::system_handle_config_change);
    }
}

#[derive(Deserialize, Debug, TypeUuid, Resource, Clone)]
#[uuid = "4f602f8f-3160-4369-a4c4-062a031ad23b"]
pub struct Config {
    pub assets: AssetsConfig,
    pub player: PlayerConfig,
    pub viewport: ViewportConfig,
}

impl Config {
    pub fn system_handle_config_change(mut commands: Commands, opt_config: Option<Res<Config>>) {
        if opt_config.is_none() {
            return;
        }
        let config = opt_config.unwrap();
        if !config.is_changed() {
            return;
        }

        info!("Config updated");

        // insert all resources derived from config
        commands.insert_resource(config.assets.clone());
        commands.insert_resource(config.viewport.clone());
        commands.insert_resource(config.player.clone());
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
            let config: Config = toml::from_str(&config_str)?;
            load_context.set_default_asset(LoadedAsset::new(config));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["toml"]
    }
}
