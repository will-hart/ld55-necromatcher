use bevy::prelude::*;

use super::{COLS, GRID_SIZE, ROWS};

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

/// Converts from a tile x/y to an array index.
/// Pretty basic but I use this in a few places so may as well consolidate it
/// so I don't randomly mess it up
pub fn tile_to_idx(x: usize, y: usize) -> usize {
    if x == usize::MAX || y == usize::MAX {
        return usize::MAX;
    }

    x + y * COLS
}

/// Converts from an array index to a tile x/y.
/// Pretty basic but I use this in a few places so may as well consolidate it
/// so I don't randomly mess it up
pub fn idx_to_tile(idx: usize) -> (usize, usize) {
    if idx == usize::MAX {
        (idx, idx)
    } else {
        let x = idx % COLS;
        let y = (idx - x) / ROWS;

        (x, y)
    }
}
