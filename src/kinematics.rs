use bevy::{
    prelude::{Bundle, Component, Plugin, Query, Res, Transform, Vec2, Vec3},
    time::Time,
};

pub struct KinematicsPlugin;

impl Plugin for KinematicsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(Self::system_velocity);
        app.add_system(Self::system_acceleration);
        app.add_system(Self::system_angular_velocity);
        app.add_system(Self::system_angular_acceleration);
        app.add_system(Self::system_linear_damping);
        app.add_system(Self::system_angular_damping);
    }
}

impl KinematicsPlugin {
    fn system_velocity(mut q: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
        let dt = time.delta_seconds();
        q.iter_mut()
            .for_each(|(mut xform, vel)| xform.translation += Vec3::from((vel.0 * dt, 0.)));
    }

    fn system_angular_velocity(mut q: Query<(&mut Transform, &AngularVelocity)>, time: Res<Time>) {
        let dt = time.delta_seconds();
        q.iter_mut()
            .for_each(|(mut xform, rvel)| xform.rotate_z(rvel.0 * dt));
    }

    fn system_acceleration(mut q: Query<(&mut Velocity, &Acceleration)>, time: Res<Time>) {
        let dt = time.delta_seconds();
        q.iter_mut().for_each(|(mut vel, acc)| vel.0 += acc.0 * dt);
    }

    fn system_angular_acceleration(
        mut q: Query<(&mut AngularVelocity, &AngularAcceleration)>,
        time: Res<Time>,
    ) {
        let dt = time.delta_seconds();
        q.iter_mut()
            .for_each(|(mut rvel, racc)| rvel.0 += racc.0 * dt);
    }

    fn system_linear_damping(mut q: Query<(&mut Velocity, &LinearDamping)>, time: Res<Time>) {
        let dt = time.delta_seconds();
        q.iter_mut()
            .for_each(|(mut vel, ldamp)| vel.0 *= (1. - ldamp.0 * dt));
    }

    fn system_angular_damping(
        mut q: Query<(&mut AngularVelocity, &AngularDamping)>,
        time: Res<Time>,
    ) {
        let dt = time.delta_seconds();
        q.iter_mut()
            .for_each(|(mut rvel, rdamp)| rvel.0 *= (1. - rdamp.0 * dt));
    }
}

#[derive(Debug, Clone, Copy, Default, Bundle)]
pub struct KinematicsBundle {
    pub velocity: Velocity,
    pub angular_velocity: AngularVelocity,
    pub acceleration: Acceleration,
    pub angular_acceleration: AngularAcceleration,
    pub linear_damping: LinearDamping,
    pub angular_damping: AngularDamping,
}

#[derive(Debug, Clone, Copy, Default, Component)]
pub struct Velocity(pub Vec2);

#[derive(Debug, Clone, Copy, Default, Component)]
pub struct AngularVelocity(pub f32);

#[derive(Debug, Clone, Copy, Default, Component)]
pub struct Acceleration(pub Vec2);

#[derive(Debug, Clone, Copy, Default, Component)]
pub struct AngularAcceleration(pub f32);

#[derive(Debug, Clone, Copy, Default, Component)]
pub struct LinearDamping(pub f32);

#[derive(Debug, Clone, Copy, Default, Component)]
pub struct AngularDamping(pub f32);
