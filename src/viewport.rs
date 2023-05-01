//! Viewport Management

use bevy::{
    prelude::{
        info, Camera2dBundle, ClearColor, Color, Commands, Component, DetectChanges,
        IntoSystemAppConfig, OrthographicProjection, Plugin, Query, Res, ResMut, Resource, With,
    },
    window::{Window, WindowResolution},
};
use serde::Deserialize;

pub struct ViewportPlugin;

impl Plugin for ViewportPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(PrimaryCameraMarker::system_spawn.on_startup());
        app.add_system(ViewportConfig::system_handle_changed);
    }
}

#[derive(Deserialize, Debug, Clone, Resource)]
pub struct ViewportConfig {
    title: String,
    fit_canvas_to_parent: bool,
    prevent_default_event_handling: bool,
    resolution: [f32; 2],
    background_color: String,
    camera_scale: f32,
}

impl ViewportConfig {
    /// handle changed viewport config by updating properties
    fn system_handle_changed(
        viewport_config_opt: Option<Res<ViewportConfig>>,
        mut windows: Query<&mut Window>,
        mut projections: Query<&mut OrthographicProjection, With<PrimaryCameraMarker>>,
        mut clear_color: ResMut<ClearColor>,
    ) {
        // check that viewport config is set
        if viewport_config_opt.is_none() {
            return;
        }
        let viewport_config = viewport_config_opt.unwrap();
        // check that viewport config has changed
        if !viewport_config.is_changed() {
            return;
        }
        info!("Viewport Config Changed:\n{:?}", viewport_config);
        // propagate config changes to viewport
        // update window
        let mut window = windows.single_mut();
        window.title = viewport_config.title.clone();
        window.fit_canvas_to_parent = viewport_config.fit_canvas_to_parent;
        window.prevent_default_event_handling = viewport_config.prevent_default_event_handling;
        window.resolution = WindowResolution::from(viewport_config.resolution);
        // update clear color
        Color::hex(viewport_config.background_color.clone())
            .ok() // only handle successful parses
            .map(|c| clear_color.0 = c); // assign parsed color to clear_color

        // update camera zoom
        projections
            .iter_mut()
            .for_each(|mut p| p.scale = viewport_config.camera_scale)
    }
}

#[derive(Debug, Clone, Component)]
pub struct PrimaryCameraMarker;

impl PrimaryCameraMarker {
    fn system_spawn(mut commands: Commands) {
        commands.spawn((PrimaryCameraMarker, Camera2dBundle::default()));
    }
}
