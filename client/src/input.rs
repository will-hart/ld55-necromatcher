use bevy::{prelude::*, window::PrimaryWindow};

use crate::core::{MainCamera, COLS, GRID_SIZE, ROWS};
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorWorldCoords>()
            .add_systems(PreUpdate, track_cursor_position);
    }
}

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
pub struct CursorWorldCoords(pub Vec2);

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

/// convert from tile x/y to a world coordinate
pub fn tile_coords(x: usize, y: usize) -> Rect {
    let xcoord = (x * GRID_SIZE) as f32 - 0.5 * (GRID_SIZE * COLS) as f32;
    let ycoord = (y * GRID_SIZE) as f32 - 0.5 * (GRID_SIZE * ROWS) as f32;

    Rect {
        min: Vec2::new(xcoord, ycoord),
        max: Vec2::new(xcoord + GRID_SIZE as f32, ycoord + GRID_SIZE as f32),
    }
}

/// convert from world coordinates to a specific tile number
pub fn world_to_tile(world_pos: Vec2) -> Option<(usize, usize)> {
    let x =
        (world_pos.x + 0.5 * (GRID_SIZE * COLS) as f32 + 0.5 * GRID_SIZE as f32) / GRID_SIZE as f32;
    let y =
        (world_pos.y + 0.5 * (GRID_SIZE * COLS) as f32 + 0.5 * GRID_SIZE as f32) / GRID_SIZE as f32;

    if x < 0. || y < 0. {
        None
    } else {
        let x = x.floor() as usize;
        let y = y.floor() as usize;

        if x >= COLS || y >= ROWS {
            return None;
        }

        Some((x, y))
    }
}
