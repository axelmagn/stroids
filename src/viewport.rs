//! Viewport Management

use bevy::prelude::Plugin;

pub struct ViewportPlugin;

impl Plugin for ViewportPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        todo!()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ViewportConfig {
    title: String,
    fit_canvas_to_parent: bool,
    prevent_default_event_handling: bool,
    resolution: [f32; 2],
    background_color: String,
}
