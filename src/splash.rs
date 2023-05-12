use bevy::{
    prelude::{
        default, AssetServer, Color, Commands, Component, IntoSystemAppConfig, OnEnter, Plugin,
        Res, Transform, Vec3,
    },
    text::{Text, Text2dBundle, TextStyle},
};

use crate::app::AppState;

#[derive(Debug)]
pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(Self::system_splash_setup.in_schedule(OnEnter(AppState::Splash)));
    }
}

impl SplashPlugin {
    fn system_splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        let title_font = asset_server.load("kenney_fonts/Kenney Future.ttf");
        // display title text
        let title_text_style = TextStyle {
            font_size: 256. + 128.,
            color: Color::WHITE,
            font: title_font,
        };
        commands.spawn((
            Text2dBundle {
                text: Text::from_section("Stroids", title_text_style),
                transform: Transform {
                    translation: Vec3::new(0., 0., 0.),
                    ..default()
                },
                ..default()
            },
            SplashCleanup,
        ));

        let prompt_font = asset_server.load("fira_sans/FiraSans-Regular.ttf");
        let prompt_text_style = TextStyle {
            font_size: 64.0,
            color: Color::WHITE,
            font: prompt_font,
        };
        commands.spawn((
            Text2dBundle {
                text: Text::from_section("Press Any Button", prompt_text_style),
                transform: Transform {
                    translation: Vec3::new(0., -256., 0.),
                    ..default()
                },
                ..default()
            },
            SplashCleanup,
        ));
    }
}

// entities with this component will be cleaned up at the end of the splash
// screen.
#[derive(Debug, Component)]
struct SplashCleanup;
