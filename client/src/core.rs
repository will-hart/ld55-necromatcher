//! Contains core functionality, like spawning a camera,
//! mapping between grid and world etc.

use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};

/// Number of rows in the grid
pub const ROWS: usize = 8;

/// Number of columns in the grid
pub const COLS: usize = 8;

/// The size of each grid square
pub const GRID_SIZE: usize = 64;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_systems(Startup, spawn_camera);
    }
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        MainCamera,
        BloomSettings::default(),
    ));
}
