//! Really simple collision: everything is a circle under the hood.

use bevy::{
    prelude::{Color, Component, Plugin, Query, Transform},
    reflect::Reflect,
};
use bevy_mod_gizmos::{draw_gizmo, Gizmo};
use serde::Deserialize;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(Self::system_draw_debug);
    }
}

impl CollisionPlugin {
    // TODO: toggle based on resource
    fn system_draw_debug(q: Query<(&Collider, &Transform)>) {
        q.for_each(|(collider, xform)| {
            draw_gizmo(Gizmo::new(
                xform.translation,
                collider.radius,
                Color::PURPLE,
            ))
        });
    }
}

#[derive(Debug, Clone, Default, Component, Deserialize, Reflect)]
pub struct Collider {
    pub radius: f32,
}

impl Collider {
    pub fn is_collision(
        entity1: (&Transform, &Collider),
        entity2: (&Transform, &Collider),
    ) -> bool {
        let dist = entity1.0.translation.distance(entity2.0.translation);
        let min_dist = entity1.1.radius + entity2.1.radius;
        dist <= min_dist
    }
}
