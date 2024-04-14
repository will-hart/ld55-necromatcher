use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{loaders::AudioFiles, AppState};

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_systems(OnEnter(AppState::Game), spawn_background_music);
    }
}

fn spawn_background_music(audio: Res<Audio>, audio_files: Res<AudioFiles>) {
    audio
        .play(audio_files.music.clone())
        .with_volume(0.15)
        .looped();
}
