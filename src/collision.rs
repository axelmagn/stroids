//! Really simple collision: everything is a circle under the hood.

use bevy::prelude::{
    Component, Entity, EventWriter, IntoSystemConfig, OnUpdate, Plugin, Query, Transform,
};

use crate::app::AppState;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(Collider::system_check_collisions.in_set(OnUpdate(AppState::InGame)));
    }
}

#[derive(Debug, Clone, Component)]
pub struct Collider {
    radius: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct CollisionEvent {
    pub a: Entity,
    pub b: Entity,
}

impl Collider {
    fn system_check_collisions(
        q: Query<(Entity, &Collider, &Transform)>,
        mut collision_events: EventWriter<CollisionEvent>,
    ) {
        let mut entities: Vec<(Entity, &Collider, &Transform)> = q.iter().collect();
        entities.sort_by(|a, b| a.0.cmp(&b.0));
        for i in 0..entities.len() {
            for j in (i + 1)..entities.len() {
                let (e1, c1, t1) = entities[i];
                let (e2, c2, t2) = entities[j];
                let dist = t1.translation.distance(t2.translation);
                let is_collision = dist <= c1.radius + c2.radius;
                if is_collision {
                    collision_events.send(CollisionEvent { a: e1, b: e2 })
                }
            }
        }
    }
}
