//! Viewport Management

use bevy::{
    math::Vec3Swizzles,
    prelude::{
        info, Camera2dBundle, ClearColor, Color, Commands, Component, DetectChanges,
        IntoSystemAppConfig, IntoSystemConfig, OnUpdate, OrthographicProjection, Plugin, Query,
        Rect, Res, ResMut, Resource, Transform, Vec2, With,
    },
    reflect::Reflect,
    window::{Window, WindowResolution},
};
use serde::Deserialize;

use crate::app::AppState;

pub struct ViewportPlugin;

impl Plugin for ViewportPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<ViewportBounds>();
        app.add_system(PrimaryCameraMarker::system_spawn.on_startup());
        app.add_system(ViewportConfig::system_handle_changed);
        app.add_system(system_update_viewport_bounded.in_set(OnUpdate(AppState::InGame)));
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
    bounding_margin: f32,
}

#[derive(Resource, Clone, Reflect, Debug)]
pub struct ViewportBounds(pub Rect);

/// Component indicating that a component is bounded to the viewport, and will repeat itself.
#[derive(Component, Clone, Debug, Reflect, Default)]
pub struct ViewportBounded;

impl ViewportConfig {
    /// handle changed viewport config by updating properties
    fn system_handle_changed(
        mut commands: Commands,
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
        info!("ViewportConfig updated");
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
            .for_each(|mut p| p.scale = viewport_config.camera_scale);

        let bounds_size = Vec2::from(viewport_config.resolution) * viewport_config.camera_scale
            + viewport_config.bounding_margin;
        let viewport_bounds = ViewportBounds(Rect::from_center_size(Vec2::ZERO, bounds_size));
        info!("Viewport bounds: {:?}", viewport_bounds);
        commands.insert_resource(viewport_bounds);
    }
}

#[derive(Debug, Clone, Component)]
pub struct PrimaryCameraMarker;

impl PrimaryCameraMarker {
    fn system_spawn(mut commands: Commands) {
        commands.spawn((PrimaryCameraMarker, Camera2dBundle::default()));
    }
}

fn system_update_viewport_bounded(
    bounds: Res<ViewportBounds>,
    mut q: Query<&mut Transform, With<ViewportBounded>>,
) {
    let bounds_size = bounds.0.size();
    for mut xform in q.iter_mut() {
        let mut bounds_pos = xform.translation.xy() - bounds.0.min;

        while bounds_pos.x < 0. {
            bounds_pos.x += bounds_size.x;
        }
        bounds_pos.x %= bounds_size.x;
        while bounds_pos.y < 0. {
            bounds_pos.y += bounds_size.y;
        }
        bounds_pos.y %= bounds_size.y;
        let pos = bounds_pos + bounds.0.min;
        xform.translation.x = pos.x;
        xform.translation.y = pos.y;
    }
}
