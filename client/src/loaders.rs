use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (load_audio_files, load_spritesheet_files));
    }
}

#[derive(Resource)]
pub struct AudioFiles {
    pub place: Handle<AudioSource>,
    pub despawn: Handle<AudioSource>,
    pub level_complete: Handle<AudioSource>,
}

fn load_audio_files(mut commands: Commands, asset_server: Res<AssetServer>) {
    let place = asset_server.load("audio/place_piece.wav");
    let despawn = asset_server.load("audio/destroy.wav");
    let level_complete = asset_server.load("audio/level_complete.wav");

    commands.insert_resource(AudioFiles {
        place,
        despawn,
        level_complete,
    });
}

#[derive(Resource)]
pub struct SpritesheetFiles {
    pub main_sheet: Handle<Image>,
}

fn load_spritesheet_files(mut commands: Commands, asset_server: Res<AssetServer>) {
    let main_sheet = asset_server.load("sprites.png");
    commands.insert_resource(SpritesheetFiles { main_sheet });
}
