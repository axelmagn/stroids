use bevy::prelude::{Commands, Handle, IntoSystemAppConfig, OnEnter, Plugin, Res, Resource};
use bevy_kira_audio::{Audio, AudioControl, AudioInstance, AudioSource};

use crate::{app::AppState, loading::AssetMap};

pub struct SoundsPlugin;

impl Plugin for SoundsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(Self::system_start_music.in_schedule(OnEnter(AppState::InGame)));
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
}

#[derive(Debug, Clone, Resource)]
struct MusicAudio(Handle<AudioInstance>);
