use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    core::{event::GameEvent, state::PlayingPiece, utils::world_to_tile, MainCamera},
    AppState,
};
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorWorldCoords>()
            .init_resource::<DisableInput>()
            .add_systems(PreUpdate, track_cursor_position)
            .add_systems(Update, start_game.run_if(in_state(AppState::Menu)))
            .add_systems(Update, handle_piece_type.run_if(in_state(AppState::Game)));
    }
}

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
pub struct CursorWorldCoords(pub Vec2);

#[derive(Resource, Default)]
pub struct DisableInput(pub bool);

fn track_cursor_position(
    mut cursor_coords: ResMut<CursorWorldCoords>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        cursor_coords.0 = world_position;
    }
}

pub fn handle_piece_type(
    cursor_coords: Res<CursorWorldCoords>,
    buttons: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    disable_input: Res<DisableInput>,
    mut playing_piece: ResMut<PlayingPiece>,
    mut state_events: EventWriter<GameEvent>,
) {
    if buttons.just_pressed(MouseButton::Right) || keyboard_input.just_pressed(KeyCode::KeyS) {
        playing_piece.0 = playing_piece.0.toggle();
    }

    if keyboard_input.just_pressed(KeyCode::KeyR) {
        state_events.send(GameEvent::Reset);
    }

    if !disable_input.0 && buttons.just_pressed(MouseButton::Left) {
        let (x, y) = world_to_tile(cursor_coords.0).unwrap_or((usize::MAX, usize::MAX));
        if x < usize::MAX && y < usize::MAX {
            info!(
                "Requested piece placement at {x}, {y} - {:?}",
                playing_piece.0
            );
            state_events.send(GameEvent::PlacePlayerPiece {
                x,
                y,
                piece_type: playing_piece.0,
            });
        }
    }
}

fn start_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(AppState::Game);
    }
}
