use bevy::{
    prelude::{
        info, Commands, Component, Handle, Image, Input, IntoSystemAppConfig, IntoSystemConfig,
        MouseButton, OnEnter, OnUpdate, Plugin, Query, Res, ResMut, Resource, Transform, Vec3,
        With,
    },
    sprite::SpriteBundle,
    utils::default,
};
use bevy_kira_audio::{Audio, AudioControl, AudioInstance, AudioSource};

use crate::{
    app::AppState, collision::Collider, input::ClickListener, loading::AssetMap,
    viewport::ViewportBounds,
};

pub struct SoundsPlugin;

impl Plugin for SoundsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(Self::system_start_music.in_schedule(OnEnter(AppState::InGame)));
        app.add_system(Self::system_spawn_sound_button.in_schedule(OnEnter(AppState::InGame)));
        app.add_system(Self::system_update_sound_state.in_set(OnUpdate(AppState::InGame)));
        app.add_system(Self::system_update_sound_button.in_set(OnUpdate(AppState::InGame)));
    }
}

impl SoundsPlugin {
    fn system_start_music(
        mut commands: Commands,
        audio: Res<Audio>,
        loaded_audio: Res<AssetMap<AudioSource>>,
    ) {
        let music = loaded_audio.0.get("music").unwrap();
        let handle = audio.play(music.clone()).looped().handle();
        commands.insert_resource(MusicAudio(handle));
    }

    fn system_spawn_sound_button(
        mut commands: Commands,
        loaded_images: Res<AssetMap<Image>>,
        viewport: Res<ViewportBounds>,
    ) {
        let sound_on = SoundOn(true);
        let texture = Self::get_sound_button_icon(sound_on.0, &loaded_images);
        let offset = 150.;
        let (x, y) = (viewport.0.min.x + offset, viewport.0.max.y - offset);
        let sound_button = (
            SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3::new(x, y, 0.),
                    ..default()
                },
                ..default()
            },
            SoundButton,
            ClickListener::default(),
            Collider { radius: 32. },
        );
        commands.spawn(sound_button);
        commands.insert_resource(sound_on);
    }

    fn system_update_sound_state(
        q: Query<&ClickListener, With<SoundButton>>,
        mut sound_on: ResMut<SoundOn>,
    ) {
        q.iter()
            .flat_map(|listener| {
                listener
                    .0
                    .get_reader()
                    .iter(&listener.0)
                    .cloned()
                    .collect::<Vec<_>>()
            })
            .for_each(|ev| {
                info!("system_update_sound_state: mouse event {:?}", ev); // debug
                if ev.just_pressed(MouseButton::Left) {
                    sound_on.0 = !sound_on.0;
                    info!("system_update_sound_state: changed sound ({})", sound_on.0);
                }
            });
    }

    fn system_update_sound_button(
        mut q: Query<&mut Handle<Image>, With<SoundButton>>,
        sound_on: Res<SoundOn>,
        loaded_images: Res<AssetMap<Image>>,
    ) {
        let new_tex = Self::get_sound_button_icon(sound_on.0, &loaded_images);
        for mut old_tex in q.iter_mut() {
            *old_tex = new_tex.clone();
        }
    }

    fn get_sound_button_icon(sound_on: bool, loaded_images: &AssetMap<Image>) -> Handle<Image> {
        if sound_on {
            loaded_images.0.get("sound_on").unwrap().clone()
        } else {
            loaded_images.0.get("sound_off").unwrap().clone()
        }
    }
}

#[derive(Debug, Clone, Resource)]
struct MusicAudio(Handle<AudioInstance>);

#[derive(Debug, Clone, Resource)]
struct SoundOn(bool);

#[derive(Debug, Clone, Component)]
struct SoundButton;
