use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_systems(Startup, load_audio_files);
    }
}

#[derive(Resource)]
pub struct AudioFiles {
    pub place: Handle<AudioSource>,
    pub despawn: Handle<AudioSource>,
}

fn load_audio_files(mut commands: Commands, asset_server: Res<AssetServer>) {
    let place = asset_server.load("audio/place_piece.wav");
    let despawn = asset_server.load("audio/destroy.wav");

    commands.insert_resource(AudioFiles { place, despawn });
}
