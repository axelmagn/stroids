use bevy::{
    prelude::{
        default, AssetServer, Color, Commands, Component, Entity, Input, IntoSystemAppConfig,
        IntoSystemConfig, MouseButton, NextState, OnEnter, OnExit, OnUpdate, Plugin, Query, Res,
        ResMut, Transform, Vec3, With,
    },
    render::view,
    text::{Text, Text2dBundle, TextStyle},
};

use crate::{app::AppState, collision::Collider, input::ClickListener, viewport::ViewportBounds};

#[derive(Debug)]
pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(Self::system_setup.in_schedule(OnEnter(AppState::Splash)));
        app.add_system(Self::system_handle_click.in_set(OnUpdate(AppState::Splash)));
        app.add_system(Self::system_cleanup.in_schedule(OnExit(AppState::Splash)));
    }
}

impl SplashPlugin {
    fn system_setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        viewport_bounds: Res<ViewportBounds>,
    ) {
        // spawn prompt text
        let prompt_font = asset_server.load("fira_sans/FiraSans-Regular.ttf");
        let prompt_text_style = TextStyle {
            font_size: 64.0,
            color: Color::WHITE,
            font: prompt_font,
        };
        commands.spawn((
            Text2dBundle {
                text: Text::from_section("Click To Begin", prompt_text_style),
                transform: Transform {
                    translation: Vec3::new(0., -256., 0.),
                    ..default()
                },
                ..default()
            },
            SplashCleanup,
        ));

        // spawn  click listener
        let radius = viewport_bounds.0.half_size().max_element();
        commands.spawn((
            ClickListener::default(),
            Collider { radius },
            Transform {
                translation: Vec3::new(0., 0., -1.),
                ..default()
            },
            SplashClickListener,
            SplashCleanup,
        ));
    }

    fn system_handle_click(
        mut q: Query<&mut ClickListener, With<SplashClickListener>>,
        mut next_state: ResMut<NextState<AppState>>,
    ) {
        q.iter_mut().for_each(|mut listener| {
            let events: Vec<_> = listener.0.drain().collect();
            if events.len() > 0 {
                next_state.set(AppState::InGame);
            }
        });
    }

    fn system_cleanup(mut commands: Commands, q: Query<Entity, With<SplashCleanup>>) {
        q.iter().for_each(|e| commands.entity(e).despawn());
    }
}

// entities with this component will be cleaned up at the end of the splash
// screen.
#[derive(Debug, Component)]
pub struct SplashCleanup;

#[derive(Debug, Component)]
pub struct SplashClickListener;
