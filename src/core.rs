//! Contains core functionality, like spawning a camera,
//! mapping between grid and world etc.

use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};

use crate::AppState;

use self::{
    event::GameEvent,
    state::{
        game_event_handler::state_mutation,
        side_effects::{side_effect_handler, spawn_sprites_for_visualisations, SideEffect},
        GameState, PlayingPiece,
    },
};

pub(crate) mod colours;
pub(crate) mod event;
pub(crate) mod state;
pub(crate) mod utils;

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
            .init_resource::<GameState>()
            .init_resource::<PlayingPiece>()
            .add_event::<GameEvent>()
            .add_event::<SideEffect>()
            .add_systems(Startup, spawn_camera)
            .add_systems(OnEnter(AppState::Game), load_level)
            .add_systems(
                Update,
                (
                    state_mutation,
                    (side_effect_handler, spawn_sprites_for_visualisations).chain(),
                )
                    .run_if(in_state(AppState::Game)),
            );
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

fn load_level(mut events: EventWriter<GameEvent>) {
    events.send(GameEvent::LoadLevel { level_id: 0 });
}
